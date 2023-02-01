// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::{
  create_window_options::CreateWindowOptions,
  event::WsiEvent,
  request::{handle_requests, Request},
};
use deno_core::anyhow;
use deno_webgpu::wgpu_core::id::SurfaceId;
use std::{
  cell::Cell, collections::HashMap, rc::Rc, sync::mpsc as std_mpsc, thread,
};
use tokio::sync::mpsc as tokio_mpsc;
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  error::{NotSupportedError, OsError},
  event_loop::{EventLoop, EventLoopProxy},
  window::{Fullscreen, WindowId},
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
  handle_requests(&event_loop, &mut request_rx, &mut windows);

  // Run the real event loop.
  event_loop.run(move |event, window_target, control_flow| {
    event_tx.blocking_send(event.into()).unwrap();
    handle_requests(window_target, &mut request_rx, &mut windows);
    control_flow.set_wait();
  });
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

  // Send a request from the proxy thread to the real event loop.
  fn send_request(&self, request: Request) {
    self.request_tx.send(request).unwrap();

    // Send an event to the real event loop if the proxy thread is currently
    // waiting to receive an event. The real event loop might be waiting on an
    // event too and won't process this request until it receives one.
    if self.waiting_for_event.get() {
      self.event_loop_proxy.send_event(()).unwrap();

      // We don't need to do this again until we request the next event.
      self.waiting_for_event.set(false);
    }
  }

  pub(crate) fn create_window(
    &self,
    options: Option<Box<CreateWindowOptions>>,
  ) -> Result<WindowId, OsError> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::CreateWindow { options, result_tx });
    result_rx.recv().unwrap()
  }

  pub(crate) fn destroy_window(&self, window_id: WindowId) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::DestroyWindow {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn create_webgpu_surface(
    &self,
    window_id: WindowId,
    webgpu_instance: Box<deno_webgpu::Instance>,
  ) -> (Box<deno_webgpu::Instance>, SurfaceId) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::CreateWebGpuSurface {
      window_id,
      webgpu_instance,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_scale_factor(&self, window_id: WindowId) -> f64 {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowScaleFactor {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_request_redraw(&self, window_id: WindowId) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowRedraw {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_inner_position(
    &self,
    window_id: WindowId,
  ) -> Result<PhysicalPosition<i32>, NotSupportedError> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowInnerPosition {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_outer_position(
    &self,
    window_id: WindowId,
  ) -> Result<PhysicalPosition<i32>, NotSupportedError> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowOuterPosition {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_outer_position(
    &self,
    window_id: WindowId,
    position: PhysicalPosition<i32>,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetOuterPosition {
      window_id,
      position,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_inner_size(
    &self,
    window_id: WindowId,
  ) -> PhysicalSize<u32> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowInnerSize {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_inner_size(
    &self,
    window_id: WindowId,
    size: PhysicalSize<u32>,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetInnerSize {
      window_id,
      size,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_outer_size(
    &self,
    window_id: WindowId,
  ) -> PhysicalSize<u32> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowOuterSize {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_min_inner_size(
    &self,
    window_id: WindowId,
    size: Option<PhysicalSize<u32>>,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetMinInnerSize {
      window_id,
      size,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_max_inner_size(
    &self,
    window_id: WindowId,
    size: Option<PhysicalSize<u32>>,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetMaxInnerSize {
      window_id,
      size,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_title(&self, window_id: WindowId, title: String) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetTitle {
      window_id,
      title,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_visible(&self, window_id: WindowId, visible: bool) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetVisible {
      window_id,
      visible,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_is_visible(&self, window_id: WindowId) -> Option<bool> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowIsVisible {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_resizable(
    &self,
    window_id: WindowId,
    resizable: bool,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetResizable {
      window_id,
      resizable,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_is_resizable(&self, window_id: WindowId) -> bool {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowIsResizable {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_minimized(
    &self,
    window_id: WindowId,
    minimized: bool,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetMinimized {
      window_id,
      minimized,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_maximized(
    &self,
    window_id: WindowId,
    maximized: bool,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetMaximized {
      window_id,
      maximized,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_is_maximized(&self, window_id: WindowId) -> bool {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowIsMaximized {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_fullscreen(
    &self,
    window_id: WindowId,
    fullscreen: Option<Fullscreen>,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetFullscreen {
      window_id,
      fullscreen,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_fullscreen(
    &self,
    window_id: WindowId,
  ) -> Option<Fullscreen> {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowFullscreen {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_decorations(
    &self,
    window_id: WindowId,
    decorations: bool,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetDecorations {
      window_id,
      decorations,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_is_decorated(&self, window_id: WindowId) -> bool {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowIsDecorated {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn window_set_always_on_top(
    &self,
    window_id: WindowId,
    always_on_top: bool,
  ) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::WindowSetAlwaysOnTop {
      window_id,
      always_on_top,
      result_tx,
    });
    result_rx.recv().unwrap()
  }

  pub(crate) fn focus_window(&self, window_id: WindowId) {
    let (result_tx, result_rx) = std_mpsc::sync_channel(0);
    self.send_request(Request::FocusWindow {
      window_id,
      result_tx,
    });
    result_rx.recv().unwrap()
  }
}
