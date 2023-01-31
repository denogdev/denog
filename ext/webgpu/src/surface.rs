// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::{texture::WebGpuTexture, WebGpuAdapter, WebGpuDevice};
use deno_core::{error::AnyError, op, OpState, Resource, ResourceId};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub struct WebGpuSurface(pub wgpu_core::id::SurfaceId);
impl Resource for WebGpuSurface {
  fn name(&self) -> Cow<str> {
    "webGPUSurface".into()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuSurfaceCapabilities {
  pub formats: Vec<wgpu_types::TextureFormat>,
  pub present_modes: Vec<GpuSurfacePresentMode>,
  pub alpha_modes: Vec<GpuSurfaceAlphaMode>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpuSurfacePresentMode {
  AutoVsync,
  AutoNoVsync,
  Fifo,
  FifoRelaxed,
  Immediate,
  Mailbox,
}

impl From<GpuSurfacePresentMode> for wgpu_types::PresentMode {
  fn from(value: GpuSurfacePresentMode) -> Self {
    match value {
      GpuSurfacePresentMode::AutoVsync => Self::AutoVsync,
      GpuSurfacePresentMode::AutoNoVsync => Self::AutoNoVsync,
      GpuSurfacePresentMode::Fifo => Self::Fifo,
      GpuSurfacePresentMode::FifoRelaxed => Self::FifoRelaxed,
      GpuSurfacePresentMode::Immediate => Self::Immediate,
      GpuSurfacePresentMode::Mailbox => Self::Mailbox,
    }
  }
}

impl From<wgpu_types::PresentMode> for GpuSurfacePresentMode {
  fn from(value: wgpu_types::PresentMode) -> Self {
    match value {
      wgpu_types::PresentMode::AutoVsync => Self::AutoVsync,
      wgpu_types::PresentMode::AutoNoVsync => Self::AutoNoVsync,
      wgpu_types::PresentMode::Fifo => Self::Fifo,
      wgpu_types::PresentMode::FifoRelaxed => Self::FifoRelaxed,
      wgpu_types::PresentMode::Immediate => Self::Immediate,
      wgpu_types::PresentMode::Mailbox => Self::Mailbox,
    }
  }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpuSurfaceAlphaMode {
  Auto,
  Opaque,
  PreMultiplied,
  PostMultiplied,
  Inherit,
}

impl From<GpuSurfaceAlphaMode> for wgpu_types::CompositeAlphaMode {
  fn from(value: GpuSurfaceAlphaMode) -> Self {
    match value {
      GpuSurfaceAlphaMode::Auto => Self::Auto,
      GpuSurfaceAlphaMode::Opaque => Self::Opaque,
      GpuSurfaceAlphaMode::PreMultiplied => Self::PreMultiplied,
      GpuSurfaceAlphaMode::PostMultiplied => Self::PostMultiplied,
      GpuSurfaceAlphaMode::Inherit => Self::Inherit,
    }
  }
}

impl From<wgpu_types::CompositeAlphaMode> for GpuSurfaceAlphaMode {
  fn from(value: wgpu_types::CompositeAlphaMode) -> Self {
    match value {
      wgpu_types::CompositeAlphaMode::Auto => Self::Auto,
      wgpu_types::CompositeAlphaMode::Opaque => Self::Opaque,
      wgpu_types::CompositeAlphaMode::PreMultiplied => Self::PreMultiplied,
      wgpu_types::CompositeAlphaMode::PostMultiplied => Self::PostMultiplied,
      wgpu_types::CompositeAlphaMode::Inherit => Self::Inherit,
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuSurfaceConfiguration {
  pub usage: wgpu_types::TextureUsages,
  pub format: wgpu_types::TextureFormat,
  pub size: wgpu_types::Extent3d,
  pub present_mode: GpuSurfacePresentMode,
  pub alpha_mode: GpuSurfaceAlphaMode,
  pub view_formats: Vec<wgpu_types::TextureFormat>,
}

fn check_suboptimal(
  status: wgpu_types::SurfaceStatus,
) -> Result<bool, AnyError> {
  use wgpu_types::SurfaceStatus::*;
  let msg = match status {
    Good => return Ok(false),
    Suboptimal => return Ok(true),
    Timeout => "Unable to get the next frame, timed out.",
    Outdated => "The surface under the swap chain has changed.",
    Lost => "The surface under the swap chain is lost.",
  };
  Err(AnyError::msg(msg))
}

#[op]
pub fn op_webgpu_surface_get_capabilities(
  state: &mut OpState,
  surface_rid: ResourceId,
  adapter_rid: ResourceId,
) -> Result<GpuSurfaceCapabilities, AnyError> {
  let instance = state.borrow::<super::Instance>();

  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let adapter_resource =
    state.resource_table.get::<WebGpuAdapter>(adapter_rid)?;
  let adapter = adapter_resource.0;

  match gfx_select!(adapter =>
   instance.surface_get_capabilities(surface, adapter)
  ) {
    Ok(caps) => Ok(GpuSurfaceCapabilities {
      formats: caps.formats,
      present_modes: caps.present_modes.into_iter().map(Into::into).collect(),
      alpha_modes: caps.alpha_modes.into_iter().map(Into::into).collect(),
    }),
    Err(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_configure(
  state: &mut OpState,
  surface_rid: ResourceId,
  device_rid: ResourceId,
  config: GpuSurfaceConfiguration,
) -> Result<(), AnyError> {
  let instance = state.borrow::<super::Instance>();

  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let device_resource = state.resource_table.get::<WebGpuDevice>(device_rid)?;
  let device = device_resource.0;

  let config = wgpu_types::SurfaceConfiguration {
    usage: config.usage,
    format: config.format,
    width: config.size.width,
    height: config.size.height,
    present_mode: config.present_mode.into(),
    alpha_mode: config.alpha_mode.into(),
    view_formats: config.view_formats,
  };

  match gfx_select!(device =>
    instance.surface_configure(surface, device, &config)
  ) {
    None => Ok(()),
    Some(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_get_current_texture(
  state: &mut OpState,
  surface_rid: ResourceId,
  device_rid: ResourceId,
) -> Result<(ResourceId, bool), AnyError> {
  let instance = state.borrow::<super::Instance>();

  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let device_resource = state.resource_table.get::<WebGpuDevice>(device_rid)?;
  let device = device_resource.0;

  match gfx_select!(device =>
    instance.surface_get_current_texture(surface, ())
  ) {
    Ok(output) => {
      let suboptimal = check_suboptimal(output.status)?;
      let texture_resource = WebGpuTexture(output.texture_id.unwrap());
      let texture_rid = state.resource_table.add(texture_resource);
      Ok((texture_rid, suboptimal))
    }
    Err(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_texture_discard(
  state: &mut OpState,
  surface_rid: ResourceId,
  device_rid: ResourceId,
) -> Result<(), AnyError> {
  let instance = state.borrow::<super::Instance>();

  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let device_resource = state.resource_table.get::<WebGpuDevice>(device_rid)?;
  let device = device_resource.0;

  match gfx_select!(device =>
    instance.surface_texture_discard(surface)
  ) {
    Ok(()) => Ok(()),
    Err(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_texture_present(
  state: &mut OpState,
  surface_rid: ResourceId,
  device_rid: ResourceId,
) -> Result<(), AnyError> {
  let instance = state.borrow::<super::Instance>();

  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let device_resource = state.resource_table.get::<WebGpuDevice>(device_rid)?;
  let device = device_resource.0;

  match gfx_select!(device =>
    instance.surface_present(surface)
  ) {
    Ok(status) => {
      check_suboptimal(status)?;
      Ok(())
    }
    Err(err) => Err(err.into()),
  }
}
