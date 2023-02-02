// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::device_ids::DeviceIds;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use winit::{
  event::{
    DeviceEvent, ElementState, Event, Force, Ime, MouseButton,
    MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent,
  },
  window::Theme,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiButtonState {
  Pressed,
  Released,
}

impl From<ElementState> for WsiButtonState {
  fn from(state: ElementState) -> Self {
    match state {
      ElementState::Pressed => Self::Pressed,
      ElementState::Released => Self::Released,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct WsiKeyCode(#[serde(with = "WsiKeyCodeDef")] VirtualKeyCode);

#[derive(Serialize)]
#[serde(rename_all = "kebab-case", remote = "VirtualKeyCode")]
enum WsiKeyCodeDef {
  #[serde(rename = "1")]
  Key1,
  #[serde(rename = "2")]
  Key2,
  #[serde(rename = "3")]
  Key3,
  #[serde(rename = "4")]
  Key4,
  #[serde(rename = "5")]
  Key5,
  #[serde(rename = "6")]
  Key6,
  #[serde(rename = "7")]
  Key7,
  #[serde(rename = "8")]
  Key8,
  #[serde(rename = "9")]
  Key9,
  #[serde(rename = "0")]
  Key0,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Escape,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  Snapshot,
  Scroll,
  Pause,
  Insert,
  Home,
  Delete,
  End,
  PageDown,
  PageUp,
  Left,
  Up,
  Right,
  Down,
  Back,
  Return,
  Space,
  Compose,
  Caret,
  Numlock,
  #[serde(rename = "numpad-0")]
  Numpad0,
  #[serde(rename = "numpad-1")]
  Numpad1,
  #[serde(rename = "numpad-2")]
  Numpad2,
  #[serde(rename = "numpad-3")]
  Numpad3,
  #[serde(rename = "numpad-4")]
  Numpad4,
  #[serde(rename = "numpad-5")]
  Numpad5,
  #[serde(rename = "numpad-6")]
  Numpad6,
  #[serde(rename = "numpad-7")]
  Numpad7,
  #[serde(rename = "numpad-8")]
  Numpad8,
  #[serde(rename = "numpad-9")]
  Numpad9,
  NumpadAdd,
  NumpadDivide,
  NumpadDecimal,
  NumpadComma,
  NumpadEnter,
  NumpadEquals,
  NumpadMultiply,
  NumpadSubtract,
  AbntC1,
  AbntC2,
  Apostrophe,
  Apps,
  Asterisk,
  At,
  Ax,
  Backslash,
  Calculator,
  Capital,
  Colon,
  Comma,
  Convert,
  Equals,
  Grave,
  Kana,
  Kanji,
  #[serde(rename = "left-alt")]
  LAlt,
  #[serde(rename = "left-bracket")]
  LBracket,
  #[serde(rename = "left-ctrl")]
  LControl,
  #[serde(rename = "left-shift")]
  LShift,
  #[serde(rename = "left-gui")]
  LWin,
  Mail,
  MediaSelect,
  MediaStop,
  Minus,
  Mute,
  MyComputer,
  NavigateForward,
  NavigateBackward,
  NextTrack,
  NoConvert,
  #[serde(rename = "oem-102")]
  OEM102,
  Period,
  PlayPause,
  Plus,
  Power,
  PrevTrack,
  #[serde(rename = "right-alt")]
  RAlt,
  #[serde(rename = "right-bracket")]
  RBracket,
  #[serde(rename = "right-ctrl")]
  RControl,
  #[serde(rename = "right-shift")]
  RShift,
  #[serde(rename = "right-gui")]
  RWin,
  Semicolon,
  Slash,
  Sleep,
  Stop,
  Sysrq,
  Tab,
  Underline,
  Unlabeled,
  VolumeDown,
  VolumeUp,
  Wake,
  WebBack,
  WebFavorites,
  WebForward,
  WebHome,
  WebRefresh,
  WebSearch,
  WebStop,
  Yen,
  Copy,
  Paste,
  Cut,
}

#[derive(Debug)]
pub enum WsiMouseButton {
  Left,
  Right,
  Middle,
  Other(u16),
}

impl From<MouseButton> for WsiMouseButton {
  fn from(button: MouseButton) -> Self {
    match button {
      MouseButton::Left => Self::Left,
      MouseButton::Right => Self::Right,
      MouseButton::Middle => Self::Middle,
      MouseButton::Other(u) => Self::Other(u),
    }
  }
}

impl Serialize for WsiMouseButton {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    match self {
      Self::Left => s.serialize_str("left"),
      Self::Right => s.serialize_str("right"),
      Self::Middle => s.serialize_str("middle"),
      &Self::Other(u) => s.serialize_u16(u),
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsiMouseDelta {
  x: f64,
  y: f64,
}

impl From<(f64, f64)> for WsiMouseDelta {
  fn from((x, y): (f64, f64)) -> Self {
    Self { x, y }
  }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WsiTouchForce {
  #[serde(rename_all = "camelCase")]
  Calibrated {
    value: f64,
    max_value: f64,
    altitude_angle: Option<f64>,
  },
  #[serde(rename_all = "camelCase")]
  Normalized { value: f64 },
}

impl From<Force> for WsiTouchForce {
  fn from(force: Force) -> Self {
    match force {
      Force::Calibrated {
        force,
        max_possible_force,
        altitude_angle,
      } => Self::Calibrated {
        value: force,
        max_value: max_possible_force,
        altitude_angle,
      },
      Force::Normalized(value) => Self::Normalized { value },
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiTouchPhase {
  Started,
  Moved,
  Ended,
  Cancelled,
}

impl From<TouchPhase> for WsiTouchPhase {
  fn from(phase: TouchPhase) -> Self {
    match phase {
      TouchPhase::Started => Self::Started,
      TouchPhase::Moved => Self::Moved,
      TouchPhase::Ended => Self::Ended,
      TouchPhase::Cancelled => Self::Cancelled,
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WsiWheelDelta {
  #[serde(rename_all = "camelCase")]
  LineDelta { x: f32, y: f32 },
  #[serde(rename_all = "camelCase")]
  PixelDelta { x: f64, y: f64 },
}

impl From<MouseScrollDelta> for WsiWheelDelta {
  fn from(delta: MouseScrollDelta) -> Self {
    match delta {
      MouseScrollDelta::LineDelta(x, y) => Self::LineDelta { x, y },
      MouseScrollDelta::PixelDelta(p) => Self::PixelDelta { x: p.x, y: p.y },
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiWindowTheme {
  Light,
  Dark,
}

impl From<Theme> for WsiWindowTheme {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Self::Light,
      Theme::Dark => Self::Dark,
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum WsiEvent {
  Internal,
  AppResumed,
  AppSuspended,
  #[serde(rename_all = "camelCase")]
  AxisInput {
    window: u64,
    device_id: u32,
    axis_id: u32,
    value: f64,
  },
  #[serde(rename_all = "camelCase")]
  CharInput {
    window: u64,
    code_point: u32,
  },
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
    button_id: u32,
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
  DeviceWheel {
    device_id: u32,
    delta: WsiWheelDelta,
  },
  #[serde(rename_all = "camelCase")]
  DroppedFile {
    window: u64,
    path: PathBuf,
  },
  #[serde(rename_all = "camelCase")]
  HoveredFile {
    window: u64,
    path: PathBuf,
  },
  #[serde(rename_all = "camelCase")]
  HoveredFileCancelled {
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
  KeyInput {
    window: u64,
    device_id: u32,
    scan_code: u32,
    key_code: Option<WsiKeyCode>,
    state: WsiButtonState,
    is_synthetic: bool,
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
  MouseWheel {
    window: u64,
    device_id: u32,
    delta: WsiWheelDelta,
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
  TouchInput {
    window: u64,
    device_id: u32,
    location: (f64, f64),
    touch_phase: WsiTouchPhase,
    touch_force: Option<WsiTouchForce>,
    finger_id: u64,
  },
  #[serde(rename_all = "camelCase")]
  TouchpadPressure {
    window: u64,
    device_id: u32,
    pressure: f32,
    click_level: i64,
  },
  #[serde(rename_all = "camelCase")]
  WindowFocused {
    window: u64,
    is_focused: bool,
  },
  #[serde(rename_all = "camelCase")]
  WindowMoved {
    window: u64,
    position: (i32, i32),
  },
  #[serde(rename_all = "camelCase")]
  WindowOccluded {
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
          WindowEvent::DroppedFile(path) => Self::DroppedFile { window, path },
          WindowEvent::HoveredFile(path) => Self::HoveredFile { window, path },
          WindowEvent::HoveredFileCancelled => {
            Self::HoveredFileCancelled { window }
          }
          WindowEvent::ReceivedCharacter(c) => Self::CharInput {
            window,
            code_point: c as u32,
          },
          WindowEvent::Focused(is_focused) => {
            Self::WindowFocused { window, is_focused }
          }
          WindowEvent::KeyboardInput {
            device_id,
            input,
            is_synthetic,
          } => Self::KeyInput {
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
          } => Self::MouseWheel {
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
          } => Self::AxisInput {
            window,
            device_id: device_ids.get(device_id),
            axis_id: axis,
            value,
          },
          WindowEvent::Touch(touch) => Self::TouchInput {
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
          WindowEvent::Occluded(is_occluded) => Self::WindowOccluded {
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
          DeviceEvent::MouseWheel { delta } => Self::DeviceWheel {
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
            button_id: button,
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
