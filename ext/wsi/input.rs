use serde::{Deserialize, Serialize, Serializer};
use winit::{
  event::{
    ElementState, Force, MouseButton, MouseScrollDelta, TouchPhase,
    VirtualKeyCode,
  },
  event_loop::DeviceEventFilter,
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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiDeviceEventFilter {
  Always,
  Unfocused,
  Never,
}

impl From<WsiDeviceEventFilter> for DeviceEventFilter {
  fn from(filter: WsiDeviceEventFilter) -> Self {
    match filter {
      WsiDeviceEventFilter::Always => Self::Always,
      WsiDeviceEventFilter::Unfocused => Self::Unfocused,
      WsiDeviceEventFilter::Never => Self::Never,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct WsiKeyCode(#[serde(with = "WsiKeyCodeDef")] pub VirtualKeyCode);

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
pub enum WsiScrollDelta {
  #[serde(rename_all = "camelCase")]
  LineDelta { x: f32, y: f32 },
  #[serde(rename_all = "camelCase")]
  PixelDelta { x: f64, y: f64 },
}

impl From<MouseScrollDelta> for WsiScrollDelta {
  fn from(delta: MouseScrollDelta) -> Self {
    match delta {
      MouseScrollDelta::LineDelta(x, y) => Self::LineDelta { x, y },
      MouseScrollDelta::PixelDelta(p) => Self::PixelDelta { x: p.x, y: p.y },
    }
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
