// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use serde::{
  ser::{SerializeMap, SerializeTuple},
  Serialize, Serializer,
};
use std::path::PathBuf;
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  event::{
    AxisId, DeviceEvent, DeviceId, ElementState, Event, Force, Ime,
    KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, StartCause,
    Touch, TouchPhase, VirtualKeyCode, WindowEvent,
  },
  window::{Theme, WindowId},
};

#[derive(Debug)]
pub enum WsiEvent {
  NewEvents(StartCause),
  WindowEvent {
    window_id: WindowId,
    event: WsiWindowEvent,
  },
  DeviceEvent {
    device_id: DeviceId,
    event: DeviceEvent,
  },
  UserEvent,
  Suspended,
  Resumed,
  MainEventsCleared,
  RedrawRequested(WindowId),
  RedrawEventsCleared,
  LoopDestroyed,
}

#[derive(Debug)]
pub enum WsiWindowEvent {
  Resized(PhysicalSize<u32>),
  Moved(PhysicalPosition<i32>),
  CloseRequested,
  Destroyed,
  DroppedFile(PathBuf),
  HoveredFile(PathBuf),
  HoveredFileCancelled,
  ReceivedCharacter(char),
  Focused(bool),
  KeyboardInput {
    device_id: DeviceId,
    input: KeyboardInput,
    is_synthetic: bool,
  },
  ModifiersChanged(ModifiersState),
  Ime(Ime),
  CursorMoved {
    device_id: DeviceId,
    position: PhysicalPosition<f64>,
  },
  CursorEntered {
    device_id: DeviceId,
  },
  CursorLeft {
    device_id: DeviceId,
  },
  MouseWheel {
    device_id: DeviceId,
    delta: MouseScrollDelta,
    phase: TouchPhase,
  },
  MouseInput {
    device_id: DeviceId,
    state: ElementState,
    button: MouseButton,
  },
  TouchpadPressure {
    device_id: DeviceId,
    pressure: f32,
    stage: i64,
  },
  AxisMotion {
    device_id: DeviceId,
    axis: AxisId,
    value: f64,
  },
  Touch(Touch),
  ScaleFactorChanged {
    scale_factor: f64,
  },
  ThemeChanged(Theme),
  Occluded(bool),
}

impl From<Event<'_, ()>> for WsiEvent {
  fn from(event: Event<()>) -> Self {
    match event {
      Event::NewEvents(start_cause) => Self::NewEvents(start_cause),
      Event::WindowEvent { window_id, event } => Self::WindowEvent {
        window_id,
        event: event.into(),
      },
      Event::DeviceEvent { device_id, event } => {
        Self::DeviceEvent { device_id, event }
      }
      Event::UserEvent(_) => Self::UserEvent,
      Event::Suspended => Self::Suspended,
      Event::Resumed => Self::Resumed,
      Event::MainEventsCleared => Self::MainEventsCleared,
      Event::RedrawRequested(window_id) => Self::RedrawRequested(window_id),
      Event::RedrawEventsCleared => Self::RedrawEventsCleared,
      Event::LoopDestroyed => Self::LoopDestroyed,
    }
  }
}

impl From<WindowEvent<'_>> for WsiWindowEvent {
  fn from(event: WindowEvent) -> Self {
    match event {
      WindowEvent::Resized(size) => Self::Resized(size),
      WindowEvent::Moved(position) => Self::Moved(position),
      WindowEvent::CloseRequested => Self::CloseRequested,
      WindowEvent::Destroyed => Self::Destroyed,
      WindowEvent::DroppedFile(path) => Self::DroppedFile(path),
      WindowEvent::HoveredFile(path) => Self::HoveredFile(path),
      WindowEvent::HoveredFileCancelled => Self::HoveredFileCancelled,
      WindowEvent::ReceivedCharacter(character) => {
        Self::ReceivedCharacter(character)
      }
      WindowEvent::Focused(is_focused) => Self::Focused(is_focused),
      WindowEvent::KeyboardInput {
        device_id,
        input,
        is_synthetic,
      } => Self::KeyboardInput {
        device_id,
        input,
        is_synthetic,
      },
      WindowEvent::ModifiersChanged(modifiers) => {
        Self::ModifiersChanged(modifiers)
      }
      WindowEvent::Ime(ime) => Self::Ime(ime),
      #[allow(deprecated)]
      WindowEvent::CursorMoved {
        device_id,
        position,
        modifiers: _,
      } => Self::CursorMoved {
        device_id,
        position,
      },
      WindowEvent::CursorEntered { device_id } => {
        Self::CursorEntered { device_id }
      }
      WindowEvent::CursorLeft { device_id } => Self::CursorLeft { device_id },
      #[allow(deprecated)]
      WindowEvent::MouseWheel {
        device_id,
        delta,
        phase,
        modifiers: _,
      } => Self::MouseWheel {
        device_id,
        delta,
        phase,
      },
      #[allow(deprecated)]
      WindowEvent::MouseInput {
        device_id,
        state,
        button,
        modifiers: _,
      } => Self::MouseInput {
        device_id,
        state,
        button,
      },
      WindowEvent::TouchpadPressure {
        device_id,
        pressure,
        stage,
      } => Self::TouchpadPressure {
        device_id,
        pressure,
        stage,
      },
      WindowEvent::AxisMotion {
        device_id,
        axis,
        value,
      } => Self::AxisMotion {
        device_id,
        axis,
        value,
      },
      WindowEvent::Touch(touch) => Self::Touch(touch),
      WindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size: _,
      } => Self::ScaleFactorChanged { scale_factor },
      WindowEvent::ThemeChanged(theme) => Self::ThemeChanged(theme),
      WindowEvent::Occluded(is_occluded) => Self::Occluded(is_occluded),
    }
  }
}

impl Serialize for WsiEvent {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    match self {
      WsiEvent::NewEvents(_) => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "new-events")?;
        s.end()
      }
      WsiEvent::WindowEvent { window_id, event } => {
        let wid = u64::from(*window_id);
        match event {
          WsiWindowEvent::Resized(size) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "window-resized")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("innerSize", &SerializeSize(*size))?;
            s.end()
          }
          WsiWindowEvent::Moved(position) => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "window-moved")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("position", &SerializePosition(*position))?;
            s.end()
          }
          WsiWindowEvent::CloseRequested => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "close-requested")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::Destroyed => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "window-destroyed")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::DroppedFile(path) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "dropped-file")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("path", path)?;
            s.end()
          }
          WsiWindowEvent::HoveredFile(path) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "hovered-file")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("path", path)?;
            s.end()
          }
          WsiWindowEvent::HoveredFileCancelled => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "hovered-file-cancelled")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::ReceivedCharacter(codepoint) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "character")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("codePoint", &(*codepoint as u32))?;
            s.end()
          }
          WsiWindowEvent::Focused(focused) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "window-focused")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("focused", focused)?;
            s.end()
          }
          #[allow(deprecated)]
          WsiWindowEvent::KeyboardInput {
            device_id,
            input:
              KeyboardInput {
                scancode,
                state,
                virtual_keycode,
                modifiers: _,
              },
            is_synthetic,
          } => {
            let mut s = s.serialize_map(Some(6))?;
            s.serialize_entry("type", "key")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("scanCode", scancode)?;
            s.serialize_entry("state", &SerializeElementState(*state))?;
            s.serialize_entry(
              "virtualKeyCode",
              &virtual_keycode.map(SerializeVirtualKeyCode),
            )?;
            s.serialize_entry("synthetic", is_synthetic)?;
            s.end()
          }
          WsiWindowEvent::ModifiersChanged(modifiers) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "modifiers-changed")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("modifiers", &modifiers.bits())?;
            s.end()
          }
          WsiWindowEvent::Ime(Ime::Enabled) => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "ime-enabled")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::Ime(Ime::Preedit(string, cursor_range)) => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "ime-preedit")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("string", string)?;
            s.serialize_entry("cursorRange", cursor_range)?;
            s.end()
          }
          WsiWindowEvent::Ime(Ime::Commit(string)) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "ime-commit")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("string", string)?;
            s.end()
          }
          WsiWindowEvent::Ime(Ime::Disabled) => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "ime-disabled")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::CursorMoved {
            device_id,
            position,
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "cursor-moved")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("position", &SerializePosition(*position))?;
            s.end()
          }
          WsiWindowEvent::CursorEntered { device_id } => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "cursor-entered")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::CursorLeft { device_id } => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "cursor-left")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "mouse-wheel")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("delta", &SerializeMouseScrollDelta(*delta))?;
            s.serialize_entry("phase", &SerializeTouchPhase(*phase))?;
            s.end()
          }
          WsiWindowEvent::MouseInput {
            device_id,
            state,
            button,
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "mouse-button")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("state", &SerializeElementState(*state))?;
            s.serialize_entry("button", &SerializeMouseButton(*button))?;
            s.end()
          }
          WsiWindowEvent::TouchpadPressure {
            device_id,
            pressure,
            stage,
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "touchpad-pressure")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("pressure", pressure)?;
            s.serialize_entry("stage", stage)?;
            s.end()
          }
          WsiWindowEvent::AxisMotion {
            device_id,
            axis,
            value,
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "axis-motion")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("axis", axis)?;
            s.serialize_entry("value", value)?;
            s.end()
          }
          WsiWindowEvent::Touch(Touch {
            device_id,
            phase,
            location,
            force,
            id,
          }) => {
            let mut s = s.serialize_map(Some(6))?;
            s.serialize_entry("type", "touch")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("phase", &SerializeTouchPhase(*phase))?;
            s.serialize_entry("location", &SerializePosition(*location))?;
            s.serialize_entry("force", &force.map(SerializeForce))?;
            s.serialize_entry("id", id)?;
            s.end()
          }
          WsiWindowEvent::ScaleFactorChanged { scale_factor } => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "scale-factor-changed")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("scaleFactor", scale_factor)?;
            s.end()
          }
          WsiWindowEvent::ThemeChanged(theme) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "theme-changed")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("theme", &SerializeTheme(*theme))?;
            s.end()
          }
          WsiWindowEvent::Occluded(occluded) => {
            let mut s = s.serialize_map(Some(3))?;
            s.serialize_entry("type", "window-occluded")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("occluded", occluded)?;
            s.end()
          }
        }
      }
      WsiEvent::DeviceEvent { device_id, event } => match event {
        DeviceEvent::Added => {
          let mut s = s.serialize_map(Some(1))?;
          s.serialize_entry("type", "device-added")?;
          s.end()
        }
        DeviceEvent::Removed => {
          let mut s = s.serialize_map(Some(1))?;
          s.serialize_entry("type", "device-removed")?;
          s.end()
        }
        DeviceEvent::MouseMotion { delta: (x, y) } => {
          let mut s = s.serialize_map(Some(2))?;
          s.serialize_entry("type", "mouse-motion")?;
          s.serialize_entry("delta", &SerializeMouseMotionDelta(*x, *y))?;
          s.end()
        }
        DeviceEvent::MouseWheel { delta } => {
          let mut s = s.serialize_map(Some(2))?;
          s.serialize_entry("type", "mouse-wheel")?;
          s.serialize_entry("delta", &SerializeMouseScrollDelta(*delta))?;
          s.end()
        }
        DeviceEvent::Motion { axis, value } => {
          let mut s = s.serialize_map(Some(3))?;
          s.serialize_entry("type", "axis-motion")?;
          s.serialize_entry("axis", axis)?;
          s.serialize_entry("value", value)?;
          s.end()
        }
        DeviceEvent::Button { button, state } => {
          let mut s = s.serialize_map(Some(3))?;
          s.serialize_entry("type", "button")?;
          s.serialize_entry("button", button)?;
          s.serialize_entry("state", &SerializeElementState(*state))?;
          s.end()
        }
        #[allow(deprecated)]
        DeviceEvent::Key(KeyboardInput {
          scancode,
          state,
          virtual_keycode,
          modifiers: _,
        }) => {
          let mut s = s.serialize_map(Some(4))?;
          s.serialize_entry("type", "key")?;
          s.serialize_entry("scanCode", scancode)?;
          s.serialize_entry("state", &SerializeElementState(*state))?;
          s.serialize_entry(
            "virtualKeyCode",
            &virtual_keycode.map(SerializeVirtualKeyCode),
          )?;
          s.end()
        }
        DeviceEvent::Text { codepoint } => {
          let mut s = s.serialize_map(Some(2))?;
          s.serialize_entry("type", "character")?;
          s.serialize_entry("codePoint", &(*codepoint as u32))?;
          s.end()
        }
      },
      WsiEvent::UserEvent => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "user")?;
        s.end()
      }
      WsiEvent::Suspended => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "suspend")?;
        s.end()
      }
      WsiEvent::Resumed => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "resume")?;
        s.end()
      }
      WsiEvent::MainEventsCleared => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "main-events-cleared")?;
        s.end()
      }
      WsiEvent::RedrawRequested(window_id) => {
        let mut s = s.serialize_map(Some(2))?;
        s.serialize_entry("type", "redraw-requested")?;
        s.serialize_entry("wid", &u64::from(*window_id))?;
        s.end()
      }
      WsiEvent::RedrawEventsCleared => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "redraw-events-cleared")?;
        s.end()
      }
      WsiEvent::LoopDestroyed => {
        let mut s = s.serialize_map(Some(1))?;
        s.serialize_entry("type", "loop-destroyed")?;
        s.end()
      }
    }
  }
}

struct SerializeElementState(ElementState);
impl Serialize for SerializeElementState {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(match self.0 {
      ElementState::Pressed => "pressed",
      ElementState::Released => "released",
    })
  }
}

struct SerializeForce(Force);
impl Serialize for SerializeForce {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    match self.0 {
      Force::Calibrated {
        force,
        max_possible_force,
        altitude_angle,
      } => {
        let mut s = s.serialize_map(Some(4))?;
        s.serialize_entry("type", "calibrated")?;
        s.serialize_entry("value", &force)?;
        s.serialize_entry("maxValue", &max_possible_force)?;
        s.serialize_entry("altitudeAngle", &altitude_angle)?;
        s.end()
      }
      Force::Normalized(value) => {
        let mut s = s.serialize_map(Some(2))?;
        s.serialize_entry("type", "normalized")?;
        s.serialize_entry("value", &value)?;
        s.end()
      }
    }
  }
}

struct SerializeMouseButton(MouseButton);
impl Serialize for SerializeMouseButton {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    match self.0 {
      MouseButton::Left => s.serialize_str("left"),
      MouseButton::Right => s.serialize_str("right"),
      MouseButton::Middle => s.serialize_str("middle"),
      MouseButton::Other(b) => s.serialize_u16(b),
    }
  }
}

struct SerializeMouseMotionDelta(f64, f64);
impl Serialize for SerializeMouseMotionDelta {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let mut s = s.serialize_map(Some(2))?;
    s.serialize_entry("x", &self.0)?;
    s.serialize_entry("y", &self.1)?;
    s.end()
  }
}

struct SerializeMouseScrollDelta(MouseScrollDelta);
impl Serialize for SerializeMouseScrollDelta {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let (t, x, y) = match self.0 {
      MouseScrollDelta::LineDelta(x, y) => ("line", x as f64, y as f64),
      MouseScrollDelta::PixelDelta(p) => ("pixel", p.x, p.y),
    };
    let mut s = s.serialize_map(Some(3))?;
    s.serialize_entry("type", t)?;
    s.serialize_entry("x", &x)?;
    s.serialize_entry("y", &y)?;
    s.end()
  }
}

struct SerializePosition<T>(PhysicalPosition<T>);
impl<T: Serialize> Serialize for SerializePosition<T> {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let mut s = s.serialize_tuple(2)?;
    s.serialize_element(&self.0.x)?;
    s.serialize_element(&self.0.y)?;
    s.end()
  }
}

struct SerializeSize<T>(PhysicalSize<T>);
impl<T: Serialize> Serialize for SerializeSize<T> {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let mut s = s.serialize_tuple(2)?;
    s.serialize_element(&self.0.width)?;
    s.serialize_element(&self.0.height)?;
    s.end()
  }
}

struct SerializeTheme(Theme);
impl Serialize for SerializeTheme {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(match self.0 {
      Theme::Light => "light",
      Theme::Dark => "dark",
    })
  }
}

struct SerializeTouchPhase(TouchPhase);
impl Serialize for SerializeTouchPhase {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(match self.0 {
      TouchPhase::Started => "started",
      TouchPhase::Moved => "moved",
      TouchPhase::Ended => "ended",
      TouchPhase::Cancelled => "cancelled",
    })
  }
}

struct SerializeVirtualKeyCode(VirtualKeyCode);
impl Serialize for SerializeVirtualKeyCode {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    use VirtualKeyCode::*;
    s.serialize_str(match self.0 {
      Key1 => "1",
      Key2 => "2",
      Key3 => "3",
      Key4 => "4",
      Key5 => "5",
      Key6 => "6",
      Key7 => "7",
      Key8 => "8",
      Key9 => "9",
      Key0 => "0",
      A => "a",
      B => "b",
      C => "c",
      D => "d",
      E => "e",
      F => "f",
      G => "g",
      H => "h",
      I => "i",
      J => "j",
      K => "k",
      L => "l",
      M => "m",
      N => "n",
      O => "o",
      P => "p",
      Q => "q",
      R => "r",
      S => "s",
      T => "t",
      U => "u",
      V => "v",
      W => "w",
      X => "x",
      Y => "y",
      Z => "z",
      Escape => "escape",
      F1 => "f1",
      F2 => "f2",
      F3 => "f3",
      F4 => "f4",
      F5 => "f5",
      F6 => "f6",
      F7 => "f7",
      F8 => "f8",
      F9 => "f9",
      F10 => "f10",
      F11 => "f11",
      F12 => "f12",
      F13 => "f13",
      F14 => "f14",
      F15 => "f15",
      F16 => "f16",
      F17 => "f17",
      F18 => "f18",
      F19 => "f19",
      F20 => "f20",
      F21 => "f21",
      F22 => "f22",
      F23 => "f23",
      F24 => "f24",
      Snapshot => "snapshot",
      Scroll => "scroll",
      Pause => "pause",
      Insert => "insert",
      Home => "home",
      Delete => "delete",
      End => "end",
      PageDown => "page-down",
      PageUp => "page-up",
      Left => "left",
      Up => "up",
      Right => "right",
      Down => "down",
      Back => "back",
      Return => "return",
      Space => "space",
      Compose => "compose",
      Caret => "caret",
      Numlock => "numlock",
      Numpad0 => "numpad-0",
      Numpad1 => "numpad-1",
      Numpad2 => "numpad-2",
      Numpad3 => "numpad-3",
      Numpad4 => "numpad-4",
      Numpad5 => "numpad-5",
      Numpad6 => "numpad-6",
      Numpad7 => "numpad-7",
      Numpad8 => "numpad-8",
      Numpad9 => "numpad-9",
      NumpadAdd => "numpad-add",
      NumpadDivide => "numpad-divide",
      NumpadDecimal => "numpad-decimal",
      NumpadComma => "numpad-comma",
      NumpadEnter => "numpad-enter",
      NumpadEquals => "numpad-equals",
      NumpadMultiply => "numpad-multiply",
      NumpadSubtract => "numpad-subtract",
      AbntC1 => "abnt-c1",
      AbntC2 => "abnt-c2",
      Apostrophe => "apostrophe",
      Apps => "apps",
      Asterisk => "asterisk",
      At => "at",
      Ax => "ax",
      Backslash => "backslash",
      Calculator => "calculator",
      Capital => "capital",
      Colon => "colon",
      Comma => "comma",
      Convert => "convert",
      Equals => "equals",
      Grave => "grave",
      Kana => "kana",
      Kanji => "kanji",
      LAlt => "left-alt",
      LBracket => "left-bracket",
      LControl => "left-control",
      LShift => "left-shift",
      LWin => "left-win",
      Mail => "mail",
      MediaSelect => "media-select",
      MediaStop => "media-stop",
      Minus => "minus",
      Mute => "mute",
      MyComputer => "my-computer",
      NavigateForward => "navigate-forward",
      NavigateBackward => "navigate-backward",
      NextTrack => "next-track",
      NoConvert => "no-convert",
      OEM102 => "oem-102",
      Period => "period",
      PlayPause => "play-pause",
      Plus => "plus",
      Power => "power",
      PrevTrack => "prev-track",
      RAlt => "right-alt",
      RBracket => "right-bracket",
      RControl => "right-control",
      RShift => "right-shift",
      RWin => "right-win",
      Semicolon => "semicolon",
      Slash => "slash",
      Sleep => "sleep",
      Stop => "stop",
      Sysrq => "sysrq",
      Tab => "tab",
      Underline => "underline",
      Unlabeled => "unlabeled",
      VolumeDown => "volume-down",
      VolumeUp => "volume-up",
      Wake => "wake",
      WebBack => "web-back",
      WebFavorites => "web-favorites",
      WebForward => "web-forward",
      WebHome => "web-home",
      WebRefresh => "web-refresh",
      WebSearch => "web-search",
      WebStop => "web-stop",
      Yen => "yen",
      Copy => "copy",
      Paste => "paste",
      Cut => "cut",
    })
  }
}
