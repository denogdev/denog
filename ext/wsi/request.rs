// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::window::WsiCreateWindowOptions;
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
  window::{
    Fullscreen, Theme, Window, WindowBuilder, WindowButtons, WindowId,
    WindowLevel,
  },
};

// A request from the proxy thread to the real event loop.
pub enum Request {
  NextEvent,

  CreateWindow {
    options: Option<Box<WsiCreateWindowOptions>>,
    result_tx: SyncSender<Result<WindowId, OsError>>,
  },

  WindowSetContentProtected {
    window_id: WindowId,
    protected: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsDecorated {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetDecorations {
    window_id: WindowId,
    decorations: bool,
    result_tx: SyncSender<()>,
  },

  WindowEnabledButtons {
    window_id: WindowId,
    result_tx: SyncSender<WindowButtons>,
  },

  WindowSetEnabledButtons {
    window_id: WindowId,
    buttons: WindowButtons,
    result_tx: SyncSender<()>,
  },

  WindowHasFocus {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  FocusWindow {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },

  WindowFullscreen {
    window_id: WindowId,
    result_tx: SyncSender<Option<Fullscreen>>,
  },

  WindowSetFullscreen {
    window_id: WindowId,
    fullscreen: Option<Fullscreen>,
    result_tx: SyncSender<()>,
  },

  CreateWebGpuSurface {
    window_id: WindowId,
    webgpu_instance: Box<deno_webgpu::Instance>,
    result_tx: SyncSender<(Box<deno_webgpu::Instance>, SurfaceId)>,
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

  WindowOuterSize {
    window_id: WindowId,
    result_tx: SyncSender<PhysicalSize<u32>>,
  },

  WindowSetInnerSize {
    window_id: WindowId,
    size: PhysicalSize<u32>,
    result_tx: SyncSender<()>,
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

  WindowSetLevel {
    window_id: WindowId,
    level: WindowLevel,
    result_tx: SyncSender<()>,
  },

  WindowIsMinimized {
    window_id: WindowId,
    result_tx: SyncSender<Option<bool>>,
  },

  WindowSetMinimized {
    window_id: WindowId,
    minimized: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsMaximized {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetMaximized {
    window_id: WindowId,
    maximized: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsResizable {
    window_id: WindowId,
    result_tx: SyncSender<bool>,
  },

  WindowSetResizable {
    window_id: WindowId,
    resizable: bool,
    result_tx: SyncSender<()>,
  },

  WindowResizeIncrements {
    window_id: WindowId,
    result_tx: SyncSender<Option<PhysicalSize<u32>>>,
  },

  WindowSetResizeIncrements {
    window_id: WindowId,
    increments: Option<PhysicalSize<u32>>,
    result_tx: SyncSender<()>,
  },

  WindowScaleFactor {
    window_id: WindowId,
    result_tx: SyncSender<f64>,
  },

  WindowTheme {
    window_id: WindowId,
    result_tx: SyncSender<Option<Theme>>,
  },

  WindowSetTheme {
    window_id: WindowId,
    theme: Option<Theme>,
    result_tx: SyncSender<()>,
  },

  WindowTitle {
    window_id: WindowId,
    result_tx: SyncSender<String>,
  },

  WindowSetTitle {
    window_id: WindowId,
    title: String,
    result_tx: SyncSender<()>,
  },

  WindowSetTransparent {
    window_id: WindowId,
    transparent: bool,
    result_tx: SyncSender<()>,
  },

  WindowIsVisible {
    window_id: WindowId,
    result_tx: SyncSender<Option<bool>>,
  },

  WindowSetVisible {
    window_id: WindowId,
    visible: bool,
    result_tx: SyncSender<()>,
  },

  WindowRedraw {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },

  DestroyWindow {
    window_id: WindowId,
    result_tx: SyncSender<()>,
  },
}

impl Debug for Request {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Request::NextEvent => f.write_str("Request::NextEvent"),

      Request::CreateWindow {
        options,
        result_tx: _,
      } => f
        .debug_struct("Request::CreateWindow")
        .field("options", options)
        .finish_non_exhaustive(),

      Request::WindowSetContentProtected {
        window_id,
        protected,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetContentProtected")
        .field("window_id", window_id)
        .field("protected", protected)
        .finish_non_exhaustive(),

      Request::WindowIsDecorated {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowIsDecorated")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetDecorations {
        window_id,
        decorations,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetDecorations")
        .field("window_id", window_id)
        .field("decorations", decorations)
        .finish_non_exhaustive(),

      Request::WindowEnabledButtons {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowEnabledButtons")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetEnabledButtons {
        window_id,
        buttons,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetEnabledButtons")
        .field("window_id", window_id)
        .field("buttons", buttons)
        .finish_non_exhaustive(),

      Request::WindowHasFocus {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowHasFocus")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::FocusWindow {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::FocusWindow")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowFullscreen {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowFullscreen")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetFullscreen {
        window_id,
        fullscreen,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetFullscreen")
        .field("window_id", window_id)
        .field("fullscreen", fullscreen)
        .finish_non_exhaustive(),

      Request::CreateWebGpuSurface {
        window_id,
        webgpu_instance: _,
        result_tx: _,
      } => f
        .debug_struct("Request::CreateWebGpuSurface")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowInnerPosition {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowInnerPosition")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowOuterPosition {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowOuterPosition")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetOuterPosition {
        window_id,
        position,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetOuterPosition")
        .field("window_id", window_id)
        .field("position", position)
        .finish_non_exhaustive(),

      Request::WindowInnerSize {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowInnerSize")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowOuterSize {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowOuterSize")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetInnerSize {
        window_id,
        size,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowSetMinInnerSize {
        window_id,
        size,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetMinInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowSetMaxInnerSize {
        window_id,
        size,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetMaxInnerSize")
        .field("window_id", window_id)
        .field("size", size)
        .finish_non_exhaustive(),

      Request::WindowSetLevel {
        window_id,
        level,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetLevel")
        .field("window_id", window_id)
        .field("level", level)
        .finish_non_exhaustive(),

      Request::WindowIsMinimized {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowIsMinimized")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetMinimized {
        window_id,
        minimized,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetMinimized")
        .field("window_id", window_id)
        .field("minimized", minimized)
        .finish_non_exhaustive(),

      Request::WindowIsMaximized {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowIsMaximized")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetMaximized {
        window_id,
        maximized,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetMaximized")
        .field("window_id", window_id)
        .field("maximized", maximized)
        .finish_non_exhaustive(),

      Request::WindowIsResizable {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowIsResizable")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetResizable {
        window_id,
        resizable,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetResizable")
        .field("window_id", window_id)
        .field("resizable", resizable)
        .finish_non_exhaustive(),

      Request::WindowResizeIncrements {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowResizeIncrements")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetResizeIncrements {
        window_id,
        increments,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetResizeIncrements")
        .field("window_id", window_id)
        .field("increments", increments)
        .finish_non_exhaustive(),

      Request::WindowScaleFactor {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowScaleFactor")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowTheme {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowTheme")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetTheme {
        window_id,
        theme,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetTheme")
        .field("window_id", window_id)
        .field("theme", theme)
        .finish_non_exhaustive(),

      Request::WindowTitle {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowTitle")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetTitle {
        window_id,
        title,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetTitle")
        .field("window_id", window_id)
        .field("title", title)
        .finish_non_exhaustive(),

      Request::WindowSetTransparent {
        window_id,
        transparent,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetTransparent")
        .field("window_id", window_id)
        .field("transparent", transparent)
        .finish_non_exhaustive(),

      Request::WindowIsVisible {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowIsVisible")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::WindowSetVisible {
        window_id,
        visible,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowSetVisible")
        .field("window_id", window_id)
        .field("visible", visible)
        .finish_non_exhaustive(),

      Request::WindowRedraw {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::WindowRedraw")
        .field("window_id", window_id)
        .finish_non_exhaustive(),

      Request::DestroyWindow {
        window_id,
        result_tx: _,
      } => f
        .debug_struct("Request::DestroyWindow")
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

      Request::WindowSetContentProtected {
        window_id,
        protected,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx
          .send(window.set_content_protected(protected))
          .unwrap();
      }

      Request::WindowIsDecorated {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_decorated()).unwrap();
      }

      Request::WindowSetDecorations {
        window_id,
        decorations,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_decorations(decorations)).unwrap();
      }

      Request::WindowEnabledButtons {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.enabled_buttons()).unwrap();
      }

      Request::WindowSetEnabledButtons {
        window_id,
        buttons,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_enabled_buttons(buttons)).unwrap();
      }

      Request::WindowHasFocus {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.has_focus()).unwrap();
      }

      Request::FocusWindow {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.focus_window()).unwrap();
      }

      Request::WindowFullscreen {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.fullscreen()).unwrap();
      }

      Request::WindowSetFullscreen {
        window_id,
        fullscreen,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_fullscreen(fullscreen)).unwrap();
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

      Request::WindowOuterSize {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.outer_size()).unwrap();
      }

      Request::WindowSetInnerSize {
        window_id,
        size,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_inner_size(size)).unwrap();
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

      Request::WindowSetLevel {
        window_id,
        level,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_window_level(level)).unwrap();
      }

      Request::WindowIsMinimized {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_minimized()).unwrap();
      }

      Request::WindowSetMinimized {
        window_id,
        minimized,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_minimized(minimized)).unwrap();
      }

      Request::WindowIsMaximized {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_maximized()).unwrap();
      }

      Request::WindowSetMaximized {
        window_id,
        maximized,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_maximized(maximized)).unwrap();
      }

      Request::WindowIsResizable {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_resizable()).unwrap();
      }

      Request::WindowSetResizable {
        window_id,
        resizable,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_resizable(resizable)).unwrap();
      }

      Request::WindowResizeIncrements {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.resize_increments()).unwrap();
      }

      Request::WindowSetResizeIncrements {
        window_id,
        increments,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx
          .send(window.set_resize_increments(increments))
          .unwrap();
      }

      Request::WindowScaleFactor {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.scale_factor()).unwrap();
      }

      Request::WindowTheme {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.theme()).unwrap();
      }

      Request::WindowSetTheme {
        window_id,
        theme,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_theme(theme)).unwrap();
      }

      Request::WindowTitle {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.title()).unwrap();
      }

      Request::WindowSetTitle {
        window_id,
        title,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_title(&title)).unwrap();
      }

      Request::WindowSetTransparent {
        window_id,
        transparent,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_transparent(transparent)).unwrap();
      }

      Request::WindowIsVisible {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.is_visible()).unwrap();
      }

      Request::WindowSetVisible {
        window_id,
        visible,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.set_visible(visible)).unwrap();
      }

      Request::WindowRedraw {
        window_id,
        result_tx,
      } => {
        let window = windows.get(&window_id).unwrap();
        result_tx.send(window.request_redraw()).unwrap();
      }

      Request::DestroyWindow {
        window_id,
        result_tx,
      } => {
        windows.remove(&window_id);
        result_tx.send(()).unwrap();
      }
    }
  }
}
