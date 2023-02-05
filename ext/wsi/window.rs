// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use serde::{self, Deserialize, Serialize};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  window::{Fullscreen, Theme, WindowBuilder, WindowButtons, WindowLevel},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiWindowLevel {
  AlwaysOnBottom,
  Normal,
  AlwaysOnTop,
}

impl From<WsiWindowLevel> for WindowLevel {
  fn from(level: WsiWindowLevel) -> Self {
    match level {
      WsiWindowLevel::AlwaysOnBottom => Self::AlwaysOnBottom,
      WsiWindowLevel::Normal => Self::Normal,
      WsiWindowLevel::AlwaysOnTop => Self::AlwaysOnTop,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsiWindowTheme {
  Light,
  Dark,
}

impl From<WsiWindowTheme> for Theme {
  fn from(theme: WsiWindowTheme) -> Self {
    match theme {
      WsiWindowTheme::Light => Self::Light,
      WsiWindowTheme::Dark => Self::Dark,
    }
  }
}

impl From<Theme> for WsiWindowTheme {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Self::Light,
      Theme::Dark => Self::Dark,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsiCreateWindowOptions {
  pub inner_size: Option<(u32, u32)>,
  pub min_inner_size: Option<(u32, u32)>,
  pub max_inner_size: Option<(u32, u32)>,
  pub position: Option<(i32, i32)>,
  pub resizable: Option<bool>,
  pub enable_buttons: Option<u32>,
  pub title: Option<String>,
  pub fullscreen: Option<bool>, // TODO
  pub maximized: Option<bool>,
  pub visible: Option<bool>,
  pub transparent: Option<bool>,
  pub decorated: Option<bool>,
  pub level: Option<WsiWindowLevel>,
  // TODO: icon
  pub theme: Option<WsiWindowTheme>,
  pub resize_increments: Option<(u32, u32)>,
  pub content_protected: Option<bool>,
  pub active: Option<bool>,
}

impl WsiCreateWindowOptions {
  pub fn into_window_builder(
    self,
    mut builder: WindowBuilder,
  ) -> WindowBuilder {
    if let Some((width, height)) = self.inner_size {
      builder = builder.with_inner_size(PhysicalSize { width, height });
    }
    if let Some((width, height)) = self.min_inner_size {
      builder = builder.with_min_inner_size(PhysicalSize { width, height });
    }
    if let Some((width, height)) = self.max_inner_size {
      builder = builder.with_max_inner_size(PhysicalSize { width, height });
    }
    if let Some((x, y)) = self.position {
      builder = builder.with_position(PhysicalPosition { x, y });
    }
    if let Some(resizable) = self.resizable {
      builder = builder.with_resizable(resizable);
    }
    if let Some(bits) = self.enable_buttons {
      let buttons = WindowButtons::from_bits_truncate(bits);
      builder = builder.with_enabled_buttons(buttons);
    }
    if let Some(title) = self.title {
      builder = builder.with_title(title);
    }
    if let Some(true) = self.fullscreen {
      builder = builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
    }
    if let Some(maximized) = self.maximized {
      builder = builder.with_maximized(maximized);
    }
    if let Some(visible) = self.visible {
      builder = builder.with_visible(visible);
    }
    if let Some(transparent) = self.transparent {
      builder = builder.with_transparent(transparent);
    }
    if let Some(decorated) = self.decorated {
      builder = builder.with_decorations(decorated);
    }
    if let Some(level) = self.level {
      builder = builder.with_window_level(level.into());
    }
    if let Some(theme) = self.theme {
      builder = builder.with_theme(Some(theme.into()));
    }
    if let Some((width, height)) = self.resize_increments {
      builder = builder.with_resize_increments(PhysicalSize { width, height });
    }
    if let Some(content_protected) = self.content_protected {
      builder = builder.with_content_protected(content_protected);
    }
    if let Some(active) = self.active {
      builder = builder.with_active(active);
    }
    builder
  }
}
