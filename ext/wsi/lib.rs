// Copyright 2023 Jo Bates. All rights reserved. MIT license.

mod create_window_options;
mod event;
pub mod event_loop;
mod request;
mod serialize_device_id;

use crate::{
  create_window_options::CreateWindowOptions,
  event::{WsiEvent, WsiWindowEvent},
  event_loop::WsiEventLoopProxy,
};
use deno_core::{anyhow, include_js_files, op, Extension, OpState, ResourceId};
use deno_webgpu::surface::WebGpuSurface;
use std::{cell::RefCell, rc::Rc};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  window::Fullscreen,
};

pub fn init(event_loop_proxy: Option<Rc<WsiEventLoopProxy>>) -> Extension {
  Extension::builder("deno_wsi")
    .dependencies(vec!["deno_webgpu", "deno_webidl"])
    .js(include_js_files!(
      prefix "denox:ext/wsi",
      "01_wsi.js",
      "02_idl_types.js",
    ))
    .ops(vec![
      op_wsi_next_event::decl(),
      op_wsi_create_window::decl(),
      op_wsi_destroy_window::decl(),
      op_wsi_create_webgpu_surface::decl(),
      op_wsi_window_scale_factor::decl(),
      op_wsi_window_request_redraw::decl(),
      op_wsi_window_inner_position::decl(),
      op_wsi_window_outer_position::decl(),
      op_wsi_window_set_outer_position::decl(),
      op_wsi_window_inner_size::decl(),
      op_wsi_window_set_inner_size::decl(),
      op_wsi_window_outer_size::decl(),
      op_wsi_window_set_min_inner_size::decl(),
      op_wsi_window_set_max_inner_size::decl(),
      op_wsi_window_set_title::decl(),
      op_wsi_window_set_visible::decl(),
      op_wsi_window_is_visible::decl(),
      op_wsi_window_set_resizable::decl(),
      op_wsi_window_is_resizable::decl(),
      op_wsi_window_set_minimized::decl(),
      op_wsi_window_set_maximized::decl(),
      op_wsi_window_is_maximized::decl(),
      op_wsi_window_set_fullscreen::decl(),
      op_wsi_window_is_fullscreen::decl(),
      op_wsi_window_set_decorated::decl(),
      op_wsi_window_is_decorated::decl(),
      op_wsi_window_set_always_on_top::decl(),
      op_wsi_focus_window::decl(),
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
      "WSI API '{}'. Only available in the main worker and the --wsi flag must be provided.",
      api_name
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
      WsiEvent::UserEvent
      | WsiEvent::WindowEvent {
        event: WsiWindowEvent::Destroyed,
        ..
      }
      | WsiEvent::LoopDestroyed => (),
      event => return Ok(event),
    }
  }
}

#[op]
fn op_wsi_create_window(
  state: &mut OpState,
  options: Option<Box<CreateWindowOptions>>,
) -> Result<u64, anyhow::Error> {
  match try_borrow_event_loop_proxy(state, "Deno.wsi.createWindow")
    .create_window(options)
  {
    Ok(window_id) => Ok(window_id.into()),
    Err(e) => Err(e.into()),
  }
}

#[op]
fn op_wsi_destroy_window(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .destroy_window(wid.into())
}

#[op]
fn op_wsi_create_webgpu_surface(state: &mut OpState, wid: u64) -> ResourceId {
  let webgpu_instance = state
    .try_take::<deno_webgpu::Instance>()
    .unwrap_or_else(deno_webgpu::create_instance);

  let (webgpu_instance, surface_id) = state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .create_webgpu_surface(wid.into(), Box::new(webgpu_instance));

  state.put(*webgpu_instance);
  state.resource_table.add(WebGpuSurface(surface_id))
}

#[op]
fn op_wsi_window_scale_factor(state: &mut OpState, wid: u64) -> f64 {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_scale_factor(wid.into())
}

#[op]
fn op_wsi_window_request_redraw(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_request_redraw(wid.into())
}

#[op]
fn op_wsi_window_inner_position(
  state: &mut OpState,
  wid: u64,
) -> Option<(i32, i32)> {
  match state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_inner_position(wid.into())
  {
    Ok(PhysicalPosition { x, y }) => Some((x, y)),
    Err(_) => None,
  }
}

#[op]
fn op_wsi_window_outer_position(
  state: &mut OpState,
  wid: u64,
) -> Option<(i32, i32)> {
  match state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_outer_position(wid.into())
  {
    Ok(PhysicalPosition { x, y }) => Some((x, y)),
    Err(_) => None,
  }
}

#[op]
fn op_wsi_window_set_outer_position(
  state: &mut OpState,
  wid: u64,
  (x, y): (i32, i32),
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_outer_position(wid.into(), PhysicalPosition { x, y })
}

#[op]
fn op_wsi_window_inner_size(state: &mut OpState, wid: u64) -> (u32, u32) {
  let PhysicalSize { width, height } = state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_inner_size(wid.into());
  (width, height)
}

#[op]
fn op_wsi_window_set_inner_size(
  state: &mut OpState,
  wid: u64,
  (width, height): (u32, u32),
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_inner_size(wid.into(), PhysicalSize { width, height })
}

#[op]
fn op_wsi_window_outer_size(state: &mut OpState, wid: u64) -> (u32, u32) {
  let PhysicalSize { width, height } = state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_outer_size(wid.into());
  (width, height)
}

#[op]
fn op_wsi_window_set_min_inner_size(
  state: &mut OpState,
  wid: u64,
  size: Option<(u32, u32)>,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_min_inner_size(
      wid.into(),
      size.map(|(width, height)| PhysicalSize { width, height }),
    )
}

#[op]
fn op_wsi_window_set_max_inner_size(
  state: &mut OpState,
  wid: u64,
  size: Option<(u32, u32)>,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_max_inner_size(
      wid.into(),
      size.map(|(width, height)| PhysicalSize { width, height }),
    )
}

#[op]
fn op_wsi_window_set_title(state: &mut OpState, wid: u64, title: String) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_title(wid.into(), title)
}

#[op]
fn op_wsi_window_set_visible(state: &mut OpState, wid: u64, visible: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_visible(wid.into(), visible)
}

#[op]
fn op_wsi_window_is_visible(state: &mut OpState, wid: u64) -> Option<bool> {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_is_visible(wid.into())
}

#[op]
fn op_wsi_window_set_resizable(state: &mut OpState, wid: u64, resizable: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_resizable(wid.into(), resizable)
}

#[op]
fn op_wsi_window_is_resizable(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_is_resizable(wid.into())
}

#[op]
fn op_wsi_window_set_minimized(state: &mut OpState, wid: u64, minimized: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_minimized(wid.into(), minimized)
}

#[op]
fn op_wsi_window_set_maximized(state: &mut OpState, wid: u64, maximized: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_maximized(wid.into(), maximized)
}

#[op]
fn op_wsi_window_is_maximized(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_is_maximized(wid.into())
}

#[op]
fn op_wsi_window_set_fullscreen(
  state: &mut OpState,
  wid: u64,
  fullscreen: bool,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_fullscreen(
      wid.into(),
      match fullscreen {
        true => Some(Fullscreen::Borderless(None)),
        false => None,
      },
    )
}

#[op]
fn op_wsi_window_is_fullscreen(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_fullscreen(wid.into())
    .is_some()
}

#[op]
fn op_wsi_window_set_decorated(state: &mut OpState, wid: u64, decorated: bool) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_decorations(wid.into(), decorated)
}

#[op]
fn op_wsi_window_is_decorated(state: &mut OpState, wid: u64) -> bool {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_is_decorated(wid.into())
}

#[op]
fn op_wsi_window_set_always_on_top(
  state: &mut OpState,
  wid: u64,
  always_on_top: bool,
) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .window_set_always_on_top(wid.into(), always_on_top)
}

#[op]
fn op_wsi_focus_window(state: &mut OpState, wid: u64) {
  state
    .borrow::<Rc<WsiEventLoopProxy>>()
    .focus_window(wid.into())
}
