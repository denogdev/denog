// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use serde::{
  ser::{SerializeMap, SerializeTuple},
  Serialize, Serializer,
};
use std::path::PathBuf;
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  event::{
    AxisId, DeviceEvent, DeviceId, ElementState, Event, Ime, KeyboardInput,
    ModifiersState, MouseButton, MouseScrollDelta, StartCause, Touch,
    TouchPhase, WindowEvent,
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
      WindowEvent::CursorMoved {
        device_id,
        position,
        ..
      } => Self::CursorMoved {
        device_id,
        position,
      },
      WindowEvent::CursorEntered { device_id } => {
        Self::CursorEntered { device_id }
      }
      WindowEvent::CursorLeft { device_id } => Self::CursorLeft { device_id },
      WindowEvent::MouseWheel {
        device_id,
        delta,
        phase,
        ..
      } => Self::MouseWheel {
        device_id,
        delta,
        phase,
      },
      WindowEvent::MouseInput {
        device_id,
        state,
        button,
        ..
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
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        Self::ScaleFactorChanged { scale_factor }
      }
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
          WsiWindowEvent::KeyboardInput {
            input:
              KeyboardInput {
                scancode, state, ..
              },
            is_synthetic,
            ..
          } => {
            let mut s = s.serialize_map(Some(5))?;
            s.serialize_entry("type", "key")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("scanCode", scancode)?;
            s.serialize_entry("state", &SerializeElementState(*state))?;
            // TODO: keyCode
            s.serialize_entry("synthetic", is_synthetic)?;
            s.end()
          }
          WsiWindowEvent::ModifiersChanged(_) => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "modifiers-changed")?;
            s.serialize_entry("wid", &wid)?;
            // TODO
            s.end()
          }
          WsiWindowEvent::Ime(_) => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "ime")?;
            s.serialize_entry("wid", &wid)?;
            // TODO
            s.end()
          }
          WsiWindowEvent::CursorMoved { position, .. } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "cursor-moved")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("position", &SerializePosition(*position))?;
            s.end()
          }
          WsiWindowEvent::CursorEntered { .. } => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "cursor-entered")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::CursorLeft { .. } => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "cursor-left")?;
            s.serialize_entry("wid", &wid)?;
            s.end()
          }
          WsiWindowEvent::MouseWheel { delta, phase, .. } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "mouse-wheel")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("delta", &SerializeMouseScrollDelta(*delta))?;
            s.serialize_entry("phase", &SerializeTouchPhase(*phase))?;
            s.end()
          }
          WsiWindowEvent::MouseInput { state, button, .. } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "mouse-button")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("state", &SerializeElementState(*state))?;
            s.serialize_entry("button", &SerializeMouseButton(*button))?;
            s.end()
          }
          WsiWindowEvent::TouchpadPressure {
            pressure, stage, ..
          } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "touchpad-pressure")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("pressure", pressure)?;
            s.serialize_entry("stage", stage)?;
            s.end()
          }
          WsiWindowEvent::AxisMotion { axis, value, .. } => {
            let mut s = s.serialize_map(Some(4))?;
            s.serialize_entry("type", "axis-motion")?;
            s.serialize_entry("wid", &wid)?;
            s.serialize_entry("axis", axis)?;
            s.serialize_entry("value", value)?;
            s.end()
          }
          WsiWindowEvent::Touch(_) => {
            let mut s = s.serialize_map(Some(2))?;
            s.serialize_entry("type", "touch")?;
            s.serialize_entry("wid", &wid)?;
            // TODO
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
      WsiEvent::DeviceEvent { event, .. } => match event {
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
        DeviceEvent::Key(KeyboardInput {
          scancode, state, ..
        }) => {
          let mut s = s.serialize_map(Some(3))?;
          s.serialize_entry("type", "key")?;
          s.serialize_entry("scanCode", scancode)?;
          s.serialize_entry("state", &SerializeElementState(*state))?;
          // TODO: keyCode
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
