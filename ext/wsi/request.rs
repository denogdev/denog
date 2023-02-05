// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::create_window_options::CreateWindowOptions;
use deno_webgpu::wgpu_core::id::SurfaceId;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{
  collections::HashMap,
  fmt::{self, Debug, Formatter},
  sync::mpsc::{Receiver, SyncSender},
};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  error::{NotSupportedError, OsError},
  event_loop::EventLoopWindowTarget,
  window::{Fullscreen, Window, WindowBuilder, WindowId},
};

// A request from the proxy thread to the real event loop.
pub enum Request {
  NextEvent,

  CreateWindow {
    options: Option<Box<CreateWindowOptions>>,
    result_tx: SyncSender<Result<WindowId, OsError>>,
  },

  DestroyWindow {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },

  CreateWebGpuSurface {
    window_id: WindowId,
    webgpu_instance: Box<deno_webgpu::Instance>,
    result_tx: SyncSender<(Box<deno_webgpu::Instance>, SurfaceId)>,
  },

  WindowScaleFactor {
    window_id: WindowId,
    result_tx: SyncSender<f64>,
  },

  WindowRedraw {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },

  WindowInnerPosition {
    window_id: WindowId,
    result_tx: SyncSender<Result<PhysicalPosition<i32>, NotSupportedError>>,
  },

  WindowOuterPosition {
    window_id: WindowId,
    result_tx: SyncSender<Result<PhysicalPosition<i32>, NotSupportedError>>,
  },

  WindowSetOuterPosition {
    window_id: WindowId,
    position: PhysicalPosition<i32>,
    result_tx: SyncSender<()>,
  },

  WindowInnerSize {
    window_id: WindowId,
    result_tx: SyncSender<PhysicalSize<u32>>,
  },

  WindowSetInnerSize {
    window_id: WindowId,
    size: PhysicalSize<u32>,
    result_tx: SyncSender<()>,
  },

  WindowOuterSize {
    window_id: WindowId,
    result_tx: SyncSender<PhysicalSize<u32>>,
  },

  WindowSetMinInnerSize {
    window_id: WindowId,
    size: Option<PhysicalSize<u32>>,
    result_tx: SyncSender<()>,
  },

  WindowSetMaxInnerSize {
    window_id: WindowId,
    size: Option<PhysicalSize<u32>>,
    result_tx: SyncSender<()>,
  },

  WindowSetTitle {
    window_id: WindowId,
    title: String,
    result_tx: SyncSender<()>,
  },

  WindowSetVisible {
    window_id: WindowId,
    visible: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsVisible {
    window_id: WindowId,
    result_tx: SyncSender<Option<bool>>,
  },

  WindowSetResizable {
    window_id: WindowId,
    resizable: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsResizable {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetMinimized {
    window_id: WindowId,
    minimized: bool,
    result_tx: SyncSender<()>,
  },

  WindowSetMaximized {
    window_id: WindowId,
    maximized: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsMaximized {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetFullscreen {
    window_id: WindowId,
    fullscreen: Option<Fullscreen>,
    result_tx: SyncSender<()>,
  },

  WindowFullscreen {
    window_id: WindowId,
    result_tx: SyncSender<Option<Fullscreen>>,
  },

  WindowSetDecorations {
    window_id: WindowId,
    decorations: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsDecorated {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetAlwaysOnTop {
    window_id: WindowId,
    always_on_top: bool,
    result_tx: SyncSender<()>,
  },

  FocusWindow {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },
}

impl Debug for Request {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Request::NextEvent => f.write_str("Request::NextEvent"),

      Request::CreateWindow { options, .. } => f
        .debug_struct("Request::CreateWindow")
        .field("options", options)
        .finish_non_exhaustive(),

      Request::DestroyWindow { window_id, .. } => f
        .debug_struct("Request::DestroyWindow")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::CreateWebGpuSurface { window_id, .. } => f
        .debug_struct("Request::CreateWebGpuSurface")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowScaleFactor { window_id, .. } => f
        .debug_struct("Request::WindowScaleFactor")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowRedraw { window_id, .. } => f
        .debug_struct("Request::WindowRedraw")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowInnerPosition { window_id, .. } => f
        .debug_struct("Request::WindowInnerPosition")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowOuterPosition { window_id, .. } => f
        .debug_struct("Request::WindowOuterPosition")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetOuterPosition {
        window_id,
        position,
        ..
      } => f
        .debug_struct("Request::WindowSetOuterPosition")
        .field("window_id", window_id)
        .field("position", position)
        .finish_non_exhaustive(),

      Request::WindowInnerSize { window_id, .. } => f
        .debug_struct("Request::WindowInnerSize")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetInnerSize {
        window_id, size, ..
      } => f
        .debug_struct("Request::WindowSetInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowOuterSize { window_id, .. } => f
        .debug_struct("Request::WindowOuterSize")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetMinInnerSize {
        window_id, size, ..
      } => f
        .debug_struct("Request::WindowSetMinInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowSetMaxInnerSize {
        window_id, size, ..
      } => f
        .debug_struct("Request::WindowSetMaxInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowSetTitle {
        window_id, title, ..
      } => f
        .debug_struct("Request::WindowSetTitle")
        .field("window_id", window_id)
        .field("title", title)
        .finish_non_exhaustive(),

      Request::WindowSetVisible {
        window_id, visible, ..
      } => f
        .debug_struct("Request::WindowSetVisible")
        .field("window_id", window_id)
        .field("visible", visible)
        .finish_non_exhaustive(),

      Request::WindowIsVisible { window_id, .. } => f
        .debug_struct("Request::WindowIsVisible")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetResizable {
        window_id,
        resizable,
        ..
      } => f
        .debug_struct("Request::WindowSetResizable")
        .field("window_id", window_id)
        .field("resizable", resizable)
        .finish_non_exhaustive(),

      Request::WindowIsResizable { window_id, .. } => f
        .debug_struct("Request::WindowIsResizable")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetMinimized {
        window_id,
        minimized,
        ..
      } => f
        .debug_struct("Request::WindowSetMinimized")
        .field("window_id", window_id)
        .field("minimized", minimized)
        .finish_non_exhaustive(),

      Request::WindowSetMaximized {
        window_id,
        maximized,
        ..
      } => f
        .debug_struct("Request::WindowSetMaximized")
        .field("window_id", window_id)
        .field("maximized", maximized)
        .finish_non_exhaustive(),

      Request::WindowIsMaximized { window_id, .. } => f
        .debug_struct("Request::WindowIsMaximized")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetFullscreen {
        window_id,
        fullscreen,
        ..
      } => f
        .debug_struct("Request::WindowSetFullscreen")
        .field("window_id", window_id)
        .field("fullscreen", fullscreen)
        .finish_non_exhaustive(),

      Request::WindowFullscreen { window_id, .. } => f
        .debug_struct("Request::WindowFullscreen")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetDecorations {
        window_id,
        decorations,
        ..
      } => f
        .debug_struct("Request::WindowSetDecorations")
        .field("window_id", window_id)
        .field("decorations", decorations)
        .finish_non_exhaustive(),

      Request::WindowIsDecorated { window_id, .. } => f
        .debug_struct("Request::WindowIsDecorated")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetAlwaysOnTop {
        window_id,
        always_on_top,
        ..
      } => f
        .debug_struct("Request::WindowSetAlwaysOnTop")
        .field("window_id", window_id)
        .field("always_on_top", always_on_top)
        .finish_non_exhaustive(),

      Request::FocusWindow { window_id, .. } => f
        .debug_struct("Request::FocusWindow")
        .field("window_id", window_id)
        .finish_non_exhaustive(),
    }
  }
}

// Handle requests in one iteration of the real event loop
// until the proxy thread is ready for the next event.
#[allow(clippy::unit_arg)]
pub fn handle_requests(
  window_target: &EventLoopWindowTarget<()>,
  request_rx: &mut Receiver<Request>,
  windows: &mut HashMap<WindowId, Window>,
) {
  loop {
    match request_rx.recv().unwrap() {
      Request::NextEvent => break,

      Request::CreateWindow { options, result_tx } => {
        let mut builder = WindowBuilder::new().with_title("Denog");
        if let Some(options) = options {
          builder = options.into_window_builder(builder);
        }
        let result = builder.build(window_target).map(|window| {
          let window_id = window.id();
          windows.insert(window_id, window);
          window_id
        });
        result_tx.send(result).unwrap();
      }

      Request::DestroyWindow {
        window_id,
        result_tx,
      } => {
        windows.remove(&window_id);
        result_tx.send(()).unwrap();
      }

      Request::CreateWebGpuSurface {
        window_id,
        webgpu_instance,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        let surface_id = webgpu_instance.instance_create_surface(
          window.raw_display_handle(),
          window.raw_window_handle(),
          (),
        );
        result_tx.send((webgpu_instance, surface_id)).unwrap();
      }

      Request::WindowScaleFactor {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.scale_factor()).unwrap();
      }

      Request::WindowRedraw {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.request_redraw()).unwrap();
      }

      Request::WindowInnerPosition {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.inner_position()).unwrap();
      }

      Request::WindowOuterPosition {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.outer_position()).unwrap();
      }

      Request::WindowSetOuterPosition {
        window_id,
        position,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_outer_position(position)).unwrap();
      }

      Request::WindowInnerSize {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.inner_size()).unwrap();
      }

      Request::WindowSetInnerSize {
        window_id,
        size,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_inner_size(size)).unwrap();
      }

      Request::WindowOuterSize {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.outer_size()).unwrap();
      }

      Request::WindowSetMinInnerSize {
        window_id,
        size,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_min_inner_size(size)).unwrap();
      }

      Request::WindowSetMaxInnerSize {
        window_id,
        size,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_max_inner_size(size)).unwrap();
      }

      Request::WindowSetTitle {
        window_id,
        title,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_title(&title)).unwrap();
      }

      Request::WindowSetVisible {
        window_id,
        visible,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_visible(visible)).unwrap();
      }

      Request::WindowIsVisible {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_visible()).unwrap();
      }

      Request::WindowSetResizable {
        window_id,
        resizable,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_resizable(resizable)).unwrap();
      }

      Request::WindowIsResizable {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_resizable()).unwrap();
      }

      Request::WindowSetMinimized {
        window_id,
        minimized,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_minimized(minimized)).unwrap();
      }

      Request::WindowSetMaximized {
        window_id,
        maximized,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_maximized(maximized)).unwrap();
      }

      Request::WindowIsMaximized {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_maximized()).unwrap();
      }

      Request::WindowSetFullscreen {
        window_id,
        fullscreen,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_fullscreen(fullscreen)).unwrap();
      }

      Request::WindowFullscreen {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.fullscreen()).unwrap();
      }

      Request::WindowSetDecorations {
        window_id,
        decorations,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_decorations(decorations)).unwrap();
      }

      Request::WindowIsDecorated {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_decorated()).unwrap();
      }

      Request::WindowSetAlwaysOnTop {
        window_id,
        always_on_top,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx
          .send(window.set_always_on_top(always_on_top))
          .unwrap();
      }

      Request::FocusWindow {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.focus_window()).unwrap();
      }
    }
  }
}
