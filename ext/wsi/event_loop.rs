// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::{
  device_ids::DeviceIds,
  event::WsiEvent,
  request::{ExecuteRequestFn, Request},
};
use deno_core::anyhow;
use std::{
  cell::Cell, collections::HashMap, rc::Rc, sync::mpsc as std_mpsc, thread,
};
use tokio::sync::mpsc as tokio_mpsc;
use winit::{
  event_loop::{EventLoop, EventLoopProxy, EventLoopWindowTarget},
  window::Window,
};

// Spawn a proxy thread and hijack the calling thread for the real event loop.
// On some platforms (e.g. macOS), this needs to be called from the main thread.
pub fn hijack_main_and_spawn_proxy<F>(f: F) -> !
where
  F: FnOnce(Rc<WsiEventLoopProxy>) + Send + 'static,
{
  // Initialize.
  let event_loop = EventLoop::new();
  let event_loop_proxy = event_loop.create_proxy();
  let (event_tx, event_rx) = tokio_mpsc::channel(1);
  let (request_tx, mut request_rx) = std_mpsc::sync_channel(1);

  // Spawn the proxy thread.
  thread::spawn(move || {
    let wsi_event_loop_proxy = Rc::new(WsiEventLoopProxy {
      event_loop_proxy,
      waiting_for_event: Cell::new(false),
      event_rx: Cell::new(Some(event_rx)),
      request_tx,
    });
    let _retain = wsi_event_loop_proxy.clone();
    f(wsi_event_loop_proxy);
  });

  // Handle requests until the proxy thread is ready for the first event.
  let mut windows = HashMap::new();
  handle_requests(&mut request_rx, &event_loop, &mut windows);

  // Run the real event loop.
  let mut device_ids = DeviceIds::new();
  event_loop.run(move |event, window_target, control_flow| {
    let event = WsiEvent::from(event, &mut device_ids);
    event_tx.blocking_send(event).unwrap();
    handle_requests(&mut request_rx, window_target, &mut windows);
    control_flow.set_wait();
  });

  // Handle requests until the proxy thread is ready for the next event.
  fn handle_requests(
    request_rx: &mut std_mpsc::Receiver<Request>,
    window_target: &EventLoopWindowTarget<()>,
    windows: &mut HashMap<u64, Window>,
  ) {
    loop {
      match request_rx.recv().unwrap() {
        Request::NextEvent => break,
        Request::Execute(f) => f(window_target, windows),
      }
    }
  }
}

// Event loop proxy.
pub struct WsiEventLoopProxy {
  event_loop_proxy: EventLoopProxy<()>,
  waiting_for_event: Cell<bool>,
  event_rx: Cell<Option<tokio_mpsc::Receiver<WsiEvent>>>,
  request_tx: std_mpsc::SyncSender<Request>,
}

impl WsiEventLoopProxy {
  // Get the next event from the real event loop.
  // Don't call this multiple times concurrently.
  pub(crate) async fn next_event(&self) -> Result<WsiEvent, anyhow::Error> {
    // Take the receiver for exclusive use.
    let Some(mut event_rx) = self.event_rx.take() else {
      return Err(anyhow::Error::msg("Receiver already in use"));
    };

    // Send the request.
    self.request_tx.send(Request::NextEvent).unwrap();

    // Async wait for the event.
    self.waiting_for_event.set(true);
    let event = event_rx.recv().await.unwrap();
    self.waiting_for_event.set(false);

    // Save the receiver for re-use.
    self.event_rx.set(Some(event_rx));

    // Return the event.
    Ok(event)
  }

  // Send an execute request from the proxy thread to the real event loop.
  fn send_execute_request(&self, f: Box<ExecuteRequestFn>) {
    self.request_tx.send(Request::Execute(f)).unwrap();

    // Send an event to the real event loop if the proxy thread is currently
    // waiting to receive an event. The real event loop might be waiting on an
    // event too and won't process this request until it receives one.
    if self.waiting_for_event.get() {
      self.event_loop_proxy.send_event(()).unwrap();

      // We don't need to do this again until we request the next event.
      self.waiting_for_event.set(false);
    }
  }

  // Execute the given function in the real event loop thread.
  pub(crate) fn execute<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&EventLoopWindowTarget<()>, &mut HashMap<u64, Window>) -> R,
    F: Send + 'static,
    R: Send + 'static,
  {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_execute_request(Box::new(move |window_target, windows| {
      result_tx.send(f(window_target, windows)).unwrap();
    }));
    result_rx.recv().unwrap()
  }

  // Execute the given function in the real event loop with the given window.
  pub(crate) fn execute_with_window<F, R>(&self, wid: u64, f: F) -> R
  where
    F: FnOnce(&Window) -> R,
    F: Send + 'static,
    R: Send + 'static,
  {
    self.execute(move |_, windows| f(windows.get(&wid).unwrap()))
  }
}
