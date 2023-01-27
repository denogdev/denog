// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use serde::{self, Deserialize};
use winit::{
  dpi::{PhysicalPosition, PhysicalSize},
  window::{Fullscreen, WindowBuilder},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWindowArgs {
  pub inner_size: Option<(u32, u32)>,
  pub min_inner_size: Option<(u32, u32)>,
  pub max_inner_size: Option<(u32, u32)>,
  pub position: Option<(i32, i32)>,
  pub resizable: Option<bool>,
  pub title: Option<String>,
  pub fullscreen: Option<bool>, // TODO
  pub maximized: Option<bool>,
  pub visible: Option<bool>,
  pub transparent: Option<bool>,
  pub decorated: Option<bool>,
  pub always_on_top: Option<bool>,
  // TODO: icon
}

impl CreateWindowArgs {
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
    if let Some(always_on_top) = self.always_on_top {
      builder = builder.with_always_on_top(always_on_top);
    }
    builder
  }
}
