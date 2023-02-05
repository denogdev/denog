// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::{
  device_ids::DeviceIds,
  input::{
    WsiButtonState, WsiKeyCode, WsiMouseButton, WsiMouseDelta, WsiScrollDelta,
    WsiTouchForce, WsiTouchPhase,
  },
  window::WsiWindowTheme,
};
use serde::Serialize;
use std::path::PathBuf;
use winit::event::{DeviceEvent, Event, Ime, WindowEvent};

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WsiEvent {
  Internal,
  AppResumed,
  AppSuspended,
  #[serde(rename_all = "camelCase")]
  CloseRequested {
    window: u64,
  },
  #[serde(rename_all = "camelCase")]
  CursorEntered {
    window: u64,
    device_id: u32,
  },
  #[serde(rename_all = "camelCase")]
  CursorLeft {
    window: u64,
    device_id: u32,
  },
  #[serde(rename_all = "camelCase")]
  CursorMoved {
    window: u64,
    device_id: u32,
    position: (f64, f64),
  },
  #[serde(rename_all = "camelCase")]
  DeviceAdded {
    device_id: u32,
  },
  #[serde(rename_all = "camelCase")]
  DeviceAxis {
    device_id: u32,
    axis_id: u32,
    value: f64,
  },
  #[serde(rename_all = "camelCase")]
  DeviceButton {
    device_id: u32,
    button: u32,
    state: WsiButtonState,
  },
  #[serde(rename_all = "camelCase")]
  DeviceChar {
    device_id: u32,
    code_point: u32,
  },
  #[serde(rename_all = "camelCase")]
  DeviceKey {
    device_id: u32,
    scan_code: u32,
    key_code: Option<WsiKeyCode>,
    state: WsiButtonState,
  },
  #[serde(rename_all = "camelCase")]
  DeviceRemoved {
    device_id: u32,
  },
  #[serde(rename_all = "camelCase")]
  DeviceScroll {
    device_id: u32,
    delta: WsiScrollDelta,
  },
  #[serde(rename_all = "camelCase")]
  FileDropped {
    window: u64,
    path: PathBuf,
  },
  #[serde(rename_all = "camelCase")]
  FileHovered {
    window: u64,
    path: PathBuf,
  },
  #[serde(rename_all = "camelCase")]
  FileLeft {
    window: u64,
  },
  #[serde(rename_all = "camelCase")]
  ImeCommit {
    window: u64,
    string: String,
  },
  #[serde(rename_all = "camelCase")]
  ImeDisabled {
    window: u64,
  },
  #[serde(rename_all = "camelCase")]
  ImeEnabled {
    window: u64,
  },
  #[serde(rename_all = "camelCase")]
  ImePreedit {
    window: u64,
    string: String,
    cursor_range: Option<(usize, usize)>,
  },
  #[serde(rename_all = "camelCase")]
  InputAxis {
    window: u64,
    device_id: u32,
    axis_id: u32,
    value: f64,
  },
  #[serde(rename_all = "camelCase")]
  InputChar {
    window: u64,
    code_point: u32,
  },
  #[serde(rename_all = "camelCase")]
  InputKey {
    window: u64,
    device_id: u32,
    scan_code: u32,
    key_code: Option<WsiKeyCode>,
    state: WsiButtonState,
    is_synthetic: bool,
  },
  #[serde(rename_all = "camelCase")]
  InputTouch {
    window: u64,
    device_id: u32,
    location: (f64, f64),
    touch_phase: WsiTouchPhase,
    touch_force: Option<WsiTouchForce>,
    finger_id: u64,
  },
  MainEventsCleared,
  #[serde(rename_all = "camelCase")]
  ModifiersChanged {
    window: u64,
    modifiers: u32,
  },
  #[serde(rename_all = "camelCase")]
  MouseButton {
    window: u64,
    device_id: u32,
    button: WsiMouseButton,
    state: WsiButtonState,
  },
  #[serde(rename_all = "camelCase")]
  MouseMotion {
    device_id: u32,
    delta: WsiMouseDelta,
  },
  #[serde(rename_all = "camelCase")]
  MouseScroll {
    window: u64,
    device_id: u32,
    delta: WsiScrollDelta,
    touch_phase: WsiTouchPhase,
  },
  NewEvents,
  RedrawEventsCleared,
  #[serde(rename_all = "camelCase")]
  RedrawRequested {
    window: u64,
  },
  #[serde(rename_all = "camelCase")]
  ScaleFactorChanged {
    window: u64,
    scale_factor: f64,
  },
  #[serde(rename_all = "camelCase")]
  SmartMagnify {
    window: u64,
    device_id: u32,
  },
  #[serde(rename_all = "camelCase")]
  TouchpadMagnify {
    window: u64,
    device_id: u32,
    delta: f64,
    touch_phase: WsiTouchPhase,
  },
  #[serde(rename_all = "camelCase")]
  TouchpadPressure {
    window: u64,
    device_id: u32,
    pressure: f32,
    click_level: i64,
  },
  #[serde(rename_all = "camelCase")]
  TouchpadRotate {
    window: u64,
    device_id: u32,
    delta: f32,
    touch_phase: WsiTouchPhase,
  },
  #[serde(rename_all = "camelCase")]
  WindowFocus {
    window: u64,
    has_focus: bool,
  },
  #[serde(rename_all = "camelCase")]
  WindowMoved {
    window: u64,
    position: (i32, i32),
  },
  #[serde(rename_all = "camelCase")]
  WindowOcclusion {
    window: u64,
    is_occluded: bool,
  },
  #[serde(rename_all = "camelCase")]
  WindowResized {
    window: u64,
    inner_size: (u32, u32),
  },
  #[serde(rename_all = "camelCase")]
  WindowThemeChanged {
    window: u64,
    theme: WsiWindowTheme,
  },
}

impl WsiEvent {
  pub fn from(event: Event<()>, device_ids: &mut DeviceIds) -> Self {
    match event {
      Event::NewEvents(_) => Self::NewEvents,
      Event::WindowEvent { window_id, event } => {
        let window = window_id.into();
        match event {
          WindowEvent::Resized(size) => Self::WindowResized {
            window,
            inner_size: (size.width, size.height),
          },
          WindowEvent::Moved(position) => Self::WindowMoved {
            window,
            position: (position.x, position.y),
          },
          WindowEvent::CloseRequested => Self::CloseRequested { window },
          WindowEvent::Destroyed => Self::Internal,
          WindowEvent::DroppedFile(path) => Self::FileDropped { window, path },
          WindowEvent::HoveredFile(path) => Self::FileHovered { window, path },
          WindowEvent::HoveredFileCancelled => Self::FileLeft { window },
          WindowEvent::ReceivedCharacter(c) => Self::InputChar {
            window,
            code_point: c as u32,
          },
          WindowEvent::Focused(has_focus) => {
            Self::WindowFocus { window, has_focus }
          }
          WindowEvent::KeyboardInput {
            device_id,
            input,
            is_synthetic,
          } => Self::InputKey {
            window,
            device_id: device_ids.get(device_id),
            scan_code: input.scancode,
            key_code: input.virtual_keycode.map(WsiKeyCode),
            state: input.state.into(),
            is_synthetic,
          },
          WindowEvent::ModifiersChanged(modifiers) => Self::ModifiersChanged {
            window,
            modifiers: modifiers.bits(),
          },
          WindowEvent::Ime(Ime::Enabled) => Self::ImeEnabled { window },
          WindowEvent::Ime(Ime::Preedit(string, cursor_range)) => {
            Self::ImePreedit {
              window,
              string,
              cursor_range,
            }
          }
          WindowEvent::Ime(Ime::Commit(string)) => {
            Self::ImeCommit { window, string }
          }
          WindowEvent::Ime(Ime::Disabled) => Self::ImeDisabled { window },
          #[allow(deprecated)]
          WindowEvent::CursorMoved {
            device_id,
            position,
            modifiers: _,
          } => Self::CursorMoved {
            window,
            device_id: device_ids.get(device_id),
            position: (position.x, position.y),
          },
          WindowEvent::CursorEntered { device_id } => Self::CursorEntered {
            window,
            device_id: device_ids.get(device_id),
          },
          WindowEvent::CursorLeft { device_id } => Self::CursorLeft {
            window,
            device_id: device_ids.get(device_id),
          },
          #[allow(deprecated)]
          WindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
            modifiers: _,
          } => Self::MouseScroll {
            window,
            device_id: device_ids.get(device_id),
            delta: delta.into(),
            touch_phase: phase.into(),
          },
          #[allow(deprecated)]
          WindowEvent::MouseInput {
            device_id,
            state,
            button,
            modifiers: _,
          } => Self::MouseButton {
            window,
            device_id: device_ids.get(device_id),
            button: button.into(),
            state: state.into(),
          },
          WindowEvent::TouchpadMagnify {
            device_id,
            delta,
            phase,
          } => Self::TouchpadMagnify {
            window,
            device_id: device_ids.get(device_id),
            delta,
            touch_phase: phase.into(),
          },
          WindowEvent::SmartMagnify { device_id } => Self::SmartMagnify {
            window,
            device_id: device_ids.get(device_id),
          },
          WindowEvent::TouchpadRotate {
            device_id,
            delta,
            phase,
          } => Self::TouchpadRotate {
            window,
            device_id: device_ids.get(device_id),
            delta,
            touch_phase: phase.into(),
          },
          WindowEvent::TouchpadPressure {
            device_id,
            pressure,
            stage,
          } => Self::TouchpadPressure {
            window,
            device_id: device_ids.get(device_id),
            pressure,
            click_level: stage,
          },
          WindowEvent::AxisMotion {
            device_id,
            axis,
            value,
          } => Self::InputAxis {
            window,
            device_id: device_ids.get(device_id),
            axis_id: axis,
            value,
          },
          WindowEvent::Touch(touch) => Self::InputTouch {
            window,
            device_id: device_ids.get(touch.device_id),
            location: (touch.location.x, touch.location.y),
            touch_phase: touch.phase.into(),
            touch_force: touch.force.map(Into::into),
            finger_id: touch.id,
          },
          WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size: _,
          } => Self::ScaleFactorChanged {
            window,
            scale_factor,
          },
          WindowEvent::ThemeChanged(theme) => Self::WindowThemeChanged {
            window,
            theme: theme.into(),
          },
          WindowEvent::Occluded(is_occluded) => Self::WindowOcclusion {
            window,
            is_occluded,
          },
        }
      }
      Event::DeviceEvent { device_id, event } => {
        let device_id = device_ids.get(device_id);
        match event {
          DeviceEvent::Added => Self::DeviceAdded { device_id },
          DeviceEvent::Removed => Self::DeviceRemoved { device_id },
          DeviceEvent::MouseMotion { delta } => Self::MouseMotion {
            device_id,
            delta: delta.into(),
          },
          DeviceEvent::MouseWheel { delta } => Self::DeviceScroll {
            device_id,
            delta: delta.into(),
          },
          DeviceEvent::Motion { axis, value } => Self::DeviceAxis {
            device_id,
            axis_id: axis,
            value,
          },
          DeviceEvent::Button { button, state } => Self::DeviceButton {
            device_id,
            button,
            state: state.into(),
          },
          DeviceEvent::Key(input) => Self::DeviceKey {
            device_id,
            scan_code: input.scancode,
            key_code: input.virtual_keycode.map(WsiKeyCode),
            state: input.state.into(),
          },
          DeviceEvent::Text { codepoint } => Self::DeviceChar {
            device_id,
            code_point: codepoint as u32,
          },
        }
      }
      Event::UserEvent(_) => Self::Internal,
      Event::Suspended => Self::AppSuspended,
      Event::Resumed => Self::AppResumed,
      Event::MainEventsCleared => Self::MainEventsCleared,
      Event::RedrawRequested(window_id) => Self::RedrawRequested {
        window: window_id.into(),
      },
      Event::RedrawEventsCleared => Self::RedrawEventsCleared,
      Event::LoopDestroyed => Self::Internal,
    }
  }
}
