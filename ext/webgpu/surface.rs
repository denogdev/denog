// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use crate::{texture::WebGpuTexture, WebGpuAdapter, WebGpuDevice};
use deno_core::{error::AnyError, op, OpState, Resource, ResourceId};
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use std::borrow::Cow;

pub struct WebGpuSurface(pub wgpu_core::id::SurfaceId);
impl Resource for WebGpuSurface {
  fn name(&self) -> Cow<str> {
    "webGPUSurface".into()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GpuSurfaceCapabilities {
  formats: Vec<wgpu_types::TextureFormat>,
  #[serde(serialize_with = "serialize_present_modes")]
  present_modes: Vec<wgpu_types::PresentMode>,
  #[serde(serialize_with = "serialize_alpha_modes")]
  alpha_modes: Vec<wgpu_types::CompositeAlphaMode>,
}

impl From<wgpu_types::SurfaceCapabilities> for GpuSurfaceCapabilities {
  fn from(caps: wgpu_types::SurfaceCapabilities) -> Self {
    Self {
      formats: caps.formats,
      present_modes: caps.present_modes,
      alpha_modes: caps.alpha_modes,
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", remote = "wgpu_types::PresentMode")]
enum GpuSurfacePresentMode {
  AutoVsync,
  AutoNoVsync,
  Fifo,
  FifoRelaxed,
  Immediate,
  Mailbox,
}

fn serialize_present_modes<S: Serializer>(
  modes: &Vec<wgpu_types::PresentMode>,
  s: S,
) -> Result<S::Ok, S::Error> {
  let mut s = s.serialize_seq(Some(modes.len()))?;
  for mode in modes {
    use wgpu_types::PresentMode::*;
    s.serialize_element(match mode {
      AutoVsync => "auto-vsync",
      AutoNoVsync => "auto-no-vsync",
      Fifo => "fifo",
      FifoRelaxed => "fifo-relaxed",
      Immediate => "immediate",
      Mailbox => "mailbox",
    })?
  }
  s.end()
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", remote = "wgpu_types::CompositeAlphaMode")]
enum GpuSurfaceAlphaMode {
  Auto,
  Opaque,
  PreMultiplied,
  PostMultiplied,
  Inherit,
}

fn serialize_alpha_modes<S: Serializer>(
  modes: &Vec<wgpu_types::CompositeAlphaMode>,
  s: S,
) -> Result<S::Ok, S::Error> {
  let mut s = s.serialize_seq(Some(modes.len()))?;
  for mode in modes {
    use wgpu_types::CompositeAlphaMode::*;
    s.serialize_element(match mode {
      Auto => "auto",
      Opaque => "opaque",
      PreMultiplied => "pre-multiplied",
      PostMultiplied => "post-multiplied",
      Inherit => "inherit",
    })?
  }
  s.end()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GpuSurfaceConfiguration {
  usage: wgpu_types::TextureUsages,
  format: wgpu_types::TextureFormat,
  size: wgpu_types::Extent3d,
  #[serde(with = "GpuSurfacePresentMode")]
  present_mode: wgpu_types::PresentMode,
  #[serde(with = "GpuSurfaceAlphaMode")]
  alpha_mode: wgpu_types::CompositeAlphaMode,
  view_formats: Vec<wgpu_types::TextureFormat>,
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
pub(crate) fn op_webgpu_surface_get_capabilities(
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
    Ok(caps) => Ok(caps.into()),
    Err(err) => Err(err.into()),
  }
}

#[op]
pub(crate) fn op_webgpu_surface_configure(
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
    present_mode: config.present_mode,
    alpha_mode: config.alpha_mode,
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
pub(crate) fn op_webgpu_surface_get_current_texture(
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
pub(crate) fn op_webgpu_surface_texture_discard(
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
pub(crate) fn op_webgpu_surface_texture_present(
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
