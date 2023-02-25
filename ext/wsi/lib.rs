// Copyright 2023 Jo Bates. All rights reserved. MIT license.

mod cursor;
mod device_ids;
mod event;
pub mod event_loop;
mod input;
mod request;
mod window;

use crate::{
  cursor::{WsiCursorGrabMode, WsiCursorIcon},
  event::WsiEvent,
  event_loop::WsiEventLoopProxy,
  input::WsiDeviceEventFilter,
  window::{
    WsiCreateWindowOptions, WsiImePurpose, WsiResizeDirection,
    WsiUserAttentionType, WsiWindowLevel, WsiWindowTheme,
  },
};
use deno_core::{anyhow, include_js_files, op, Extension, OpState, ResourceId};
use deno_webgpu::surface::WebGpuSurface;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{cell::RefCell, rc::Rc};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  window::{Fullscreen, WindowBuilder, WindowButtons},
};

pub fn init(event_loop_proxy: Option<Rc<WsiEventLoopProxy>>) -> Extension {
  Extension::builder("deno_wsi")
    .dependencies(vec!["deno_webgpu", "deno_webidl"])
    .esm(include_js_files!("01_wsi.js", "02_idl_types.js",))
    .ops(vec![
      op_wsi_next_event::decl(),
      op_wsi_set_device_event_filter::decl(),
      op_wsi_create_window::decl(),
      op_wsi_window_set_content_protected::decl(),
      op_wsi_window_set_cursor_grab_mode::decl(),
      op_wsi_window_set_cursor_hit_test_enabled::decl(),
      op_wsi_window_set_cursor_icon::decl(),
      op_wsi_window_set_cursor_position::decl(),
      op_wsi_window_set_cursor_visible::decl(),
      op_wsi_window_is_decorated::decl(),
      op_wsi_window_set_decorated::decl(),
      op_wsi_window_get_enabled_buttons::decl(),
      op_wsi_window_set_enabled_buttons::decl(),
      op_wsi_window_has_focus::decl(),
      op_wsi_window_take_focus::decl(),
      op_wsi_window_is_fullscreen::decl(),
      op_wsi_window_set_fullscreen::decl(),
      op_wsi_window_create_gpu_surface::decl(),
      op_wsi_window_set_ime_allowed::decl(),
      op_wsi_window_set_ime_position::decl(),
      op_wsi_window_set_ime_purpose::decl(),
      op_wsi_window_get_inner_position::decl(),
      op_wsi_window_get_outer_position::decl(),
      op_wsi_window_set_outer_position::decl(),
      op_wsi_window_get_inner_size::decl(),
      op_wsi_window_get_outer_size::decl(),
      op_wsi_window_set_inner_size::decl(),
      op_wsi_window_set_min_inner_size::decl(),
      op_wsi_window_set_max_inner_size::decl(),
      op_wsi_window_set_level::decl(),
      op_wsi_window_is_minimized::decl(),
      op_wsi_window_set_minimized::decl(),
      op_wsi_window_is_maximized::decl(),
      op_wsi_window_set_maximized::decl(),
      op_wsi_window_is_resizable::decl(),
      op_wsi_window_set_resizable::decl(),
      op_wsi_window_get_resize_increments::decl(),
      op_wsi_window_set_resize_increments::decl(),
      op_wsi_window_get_scale_factor::decl(),
      op_wsi_window_get_theme::decl(),
      op_wsi_window_set_theme::decl(),
      op_wsi_window_get_title::decl(),
      op_wsi_window_set_title::decl(),
      op_wsi_window_set_transparent::decl(),
      op_wsi_window_is_visible::decl(),
      op_wsi_window_set_visible::decl(),
      op_wsi_window_begin_drag_move::decl(),
      op_wsi_window_begin_drag_resize::decl(),
      op_wsi_window_request_redraw::decl(),
      op_wsi_window_request_user_attention::decl(),
      op_wsi_window_destroy::decl(),
    ])
    .state(move |state| {
      if let Some(event_loop_proxy) = &event_loop_proxy {
        state.put(event_loop_proxy.clone());
      }
      Ok(())
    })
    .build()
}

fn try_borrow_event_loop_proxy<'a>(
  state: &'a OpState,
  api_name: &str,
) -> &'a Rc<WsiEventLoopProxy> {
  state.try_borrow::<Rc<WsiEventLoopProxy>>().unwrap_or_else(|| {
    eprintln!(
      "WSI API '{api_name}'. Only available in the main worker and the --wsi flag must be provided.",
    );
    std::process::exit(70);
  })
}

#[op]
async fn op_wsi_next_event(
  state: Rc<RefCell<OpState>>,
) -> Result<WsiEvent, anyhow::Error> {
  let proxy =
    try_borrow_event_loop_proxy(&state.borrow(), "Deno.wsi.nextEvent").clone();
  loop {
    match proxy.next_event().await? {
      WsiEvent::Internal => continue,
      event => return Ok(event),
    }
  }
}

#[op]
fn op_wsi_set_device_event_filter(
  state: &mut OpState,
  filter: WsiDeviceEventFilter,
) {
  try_borrow_event_loop_proxy(state, "Deno.wsi.setDeviceEventFilter").execute(
    |window_target, _| window_target.set_device_event_filter(filter.into()),
  )
}

#[op]
fn op_wsi_create_window(
  state: &mut OpState,
  options: Option<WsiCreateWindowOptions>,
) -> Result<u64, anyhow::Error> {
  try_borrow_event_loop_proxy(state, "Deno.wsi.createWindow")
    .execute(|window_target, windows| {
      let mut builder = WindowBuilder::new().with_title("Denog");
      if let Some(options) = options {
        builder = options.into_window_builder(builder);
      }
      builder.build(window_target).map(|window| {
        let wid = window.id().into();
        windows.insert(wid, window);
        wid
      })
    })
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_set_content_protected(
  state: &mut OpState,
  wid: u64,
  protected: bool,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_content_protected(protected)
    })
}

#[op]
fn op_wsi_window_set_cursor_grab_mode(
  state: &mut OpState,
  wid: u64,
  mode: WsiCursorGrabMode,
) -> Result<(), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_cursor_grab(mode.0))
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_set_cursor_hit_test_enabled(
  state: &mut OpState,
  wid: u64,
  enabled: bool,
) -> Result<(), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_cursor_hittest(enabled))
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_set_cursor_icon(
  state: &mut OpState,
  wid: u64,
  icon: WsiCursorIcon,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_cursor_icon(icon.0))
}

#[op]
fn op_wsi_window_set_cursor_position(
  state: &mut OpState,
  wid: u64,
  (x, y): (i32, i32),
) -> Result<(), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_cursor_position(PhysicalPosition { x, y })
    })
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_set_cursor_visible(
  state: &mut OpState,
  wid: u64,
  visible: bool,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_cursor_visible(visible))
}

#[op]
fn op_wsi_window_is_decorated(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.is_decorated())
}

#[op]
fn op_wsi_window_set_decorated(state: &mut OpState, wid: u64, decorated: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_decorations(decorated))
}

#[op]
fn op_wsi_window_get_enabled_buttons(state: &mut OpState, wid: u64) -> u32 {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.enabled_buttons().bits())
}

#[op]
fn op_wsi_window_set_enabled_buttons(
  state: &mut OpState,
  wid: u64,
  buttons: u32,
) {
  state.borrow::<Rc<WsiEventLoopProxy>>().execute_with_window(
    wid,
    move |window| {
      window.set_enabled_buttons(WindowButtons::from_bits_truncate(buttons))
    },
  )
}

#[op]
fn op_wsi_window_has_focus(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.has_focus())
}

#[op]
fn op_wsi_window_take_focus(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.focus_window())
}

#[op]
fn op_wsi_window_is_fullscreen(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.fullscreen().is_some())
}

#[op]
fn op_wsi_window_set_fullscreen(
  state: &mut OpState,
  wid: u64,
  fullscreen: bool,
) {
  state.borrow::<Rc<WsiEventLoopProxy>>().execute_with_window(
    wid,
    move |window| {
      window.set_fullscreen(match fullscreen {
        true => Some(Fullscreen::Borderless(None)),
        false => None,
      })
    },
  )
}

#[op]
fn op_wsi_window_create_gpu_surface(
  state: &mut OpState,
  wid: u64,
) -> ResourceId {
  let webgpu_instance = state
    .try_take::<deno_webgpu::Instance>()
    .unwrap_or_else(deno_webgpu::create_instance);

  let (webgpu_instance, surface_id) = state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| {
      let surface_id = webgpu_instance.instance_create_surface(
        window.raw_display_handle(),
        window.raw_window_handle(),
        (),
      );
      (webgpu_instance, surface_id)
    });

  state.put(webgpu_instance);
  state.resource_table.add(WebGpuSurface(surface_id))
}

#[op]
fn op_wsi_window_set_ime_allowed(state: &mut OpState, wid: u64, allowed: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_ime_allowed(allowed))
}

#[op]
fn op_wsi_window_set_ime_position(
  state: &mut OpState,
  wid: u64,
  (x, y): (i32, i32),
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_ime_position(PhysicalPosition { x, y })
    })
}

#[op]
fn op_wsi_window_set_ime_purpose(
  state: &mut OpState,
  wid: u64,
  purpose: WsiImePurpose,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_ime_purpose(purpose.into())
    })
}

#[op]
fn op_wsi_window_get_inner_position(
  state: &mut OpState,
  wid: u64,
) -> Result<(i32, i32), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.inner_position())
    .map(|PhysicalPosition { x, y }| (x, y))
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_get_outer_position(
  state: &mut OpState,
  wid: u64,
) -> Result<(i32, i32), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.outer_position())
    .map(|PhysicalPosition { x, y }| (x, y))
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_set_outer_position(
  state: &mut OpState,
  wid: u64,
  (x, y): (i32, i32),
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_outer_position(PhysicalPosition { x, y })
    })
}

#[op]
fn op_wsi_window_get_inner_size(state: &mut OpState, wid: u64) -> (u32, u32) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| {
      let size = window.inner_size();
      (size.width, size.height)
    })
}

#[op]
fn op_wsi_window_get_outer_size(state: &mut OpState, wid: u64) -> (u32, u32) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| {
      let size = window.outer_size();
      (size.width, size.height)
    })
}

#[op]
fn op_wsi_window_set_inner_size(
  state: &mut OpState,
  wid: u64,
  (width, height): (u32, u32),
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.set_inner_size(PhysicalSize { width, height })
    })
}

#[op]
fn op_wsi_window_set_min_inner_size(
  state: &mut OpState,
  wid: u64,
  size: Option<(u32, u32)>,
) {
  state.borrow::<Rc<WsiEventLoopProxy>>().execute_with_window(
    wid,
    move |window| {
      window.set_min_inner_size(
        size.map(|(width, height)| PhysicalSize { width, height }),
      )
    },
  )
}

#[op]
fn op_wsi_window_set_max_inner_size(
  state: &mut OpState,
  wid: u64,
  size: Option<(u32, u32)>,
) {
  state.borrow::<Rc<WsiEventLoopProxy>>().execute_with_window(
    wid,
    move |window| {
      window.set_max_inner_size(
        size.map(|(width, height)| PhysicalSize { width, height }),
      )
    },
  )
}

#[op]
fn op_wsi_window_set_level(
  state: &mut OpState,
  wid: u64,
  level: WsiWindowLevel,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.set_window_level(level.into()))
}

#[op]
fn op_wsi_window_is_minimized(state: &mut OpState, wid: u64) -> Option<bool> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.is_minimized())
}

#[op]
fn op_wsi_window_set_minimized(state: &mut OpState, wid: u64, minimized: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_minimized(minimized))
}

#[op]
fn op_wsi_window_is_maximized(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.is_maximized())
}

#[op]
fn op_wsi_window_set_maximized(state: &mut OpState, wid: u64, maximized: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_maximized(maximized))
}

#[op]
fn op_wsi_window_is_resizable(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.is_resizable())
}

#[op]
fn op_wsi_window_set_resizable(state: &mut OpState, wid: u64, resizable: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_resizable(resizable))
}

#[op]
fn op_wsi_window_get_resize_increments(
  state: &mut OpState,
  wid: u64,
) -> Option<(u32, u32)> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.resize_increments())
    .map(|PhysicalSize { width, height }| (width, height))
}

#[op]
fn op_wsi_window_set_resize_increments(
  state: &mut OpState,
  wid: u64,
  increments: Option<(u32, u32)>,
) {
  state.borrow::<Rc<WsiEventLoopProxy>>().execute_with_window(
    wid,
    move |window| {
      window.set_resize_increments(
        increments.map(|(width, height)| PhysicalSize { width, height }),
      )
    },
  )
}

#[op]
fn op_wsi_window_get_scale_factor(state: &mut OpState, wid: u64) -> f64 {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.scale_factor())
}

#[op]
fn op_wsi_window_get_theme(
  state: &mut OpState,
  wid: u64,
) -> Option<WsiWindowTheme> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.theme().map(Into::into))
}

#[op]
fn op_wsi_window_set_theme(
  state: &mut OpState,
  wid: u64,
  theme: Option<WsiWindowTheme>,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.set_theme(theme.map(Into::into)))
}

#[op]
fn op_wsi_window_get_title(state: &mut OpState, wid: u64) -> String {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.title())
}

#[op]
fn op_wsi_window_set_title(state: &mut OpState, wid: u64, title: String) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_title(&title))
}

#[op]
fn op_wsi_window_set_transparent(
  state: &mut OpState,
  wid: u64,
  transparent: bool,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_transparent(transparent))
}

#[op]
fn op_wsi_window_is_visible(state: &mut OpState, wid: u64) -> Option<bool> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.is_visible())
}

#[op]
fn op_wsi_window_set_visible(state: &mut OpState, wid: u64, visible: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| window.set_visible(visible))
}

#[op]
fn op_wsi_window_begin_drag_move(
  state: &mut OpState,
  wid: u64,
) -> Result<(), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.drag_window())
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_begin_drag_resize(
  state: &mut OpState,
  wid: u64,
  direction: WsiResizeDirection,
) -> Result<(), anyhow::Error> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.drag_resize_window(direction.into())
    })
    .map_err(Into::into)
}

#[op]
fn op_wsi_window_request_redraw(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, |window| window.request_redraw())
}

#[op]
fn op_wsi_window_request_user_attention(
  state: &mut OpState,
  wid: u64,
  attention_type: Option<WsiUserAttentionType>,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute_with_window(wid, move |window| {
      window.request_user_attention(attention_type.map(Into::into))
    })
}

#[op]
fn op_wsi_window_destroy(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .execute(move |_, windows| {
      windows.remove(&wid);
    })
}
