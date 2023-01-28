// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::op;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct WebGpuSurface(pub wgpu_core::id::SurfaceId);
impl Resource for WebGpuSurface {
  fn name(&self) -> Cow<str> {
    "webGPUSurface".into()
  }
}

#[op]
pub fn op_webgpu_surface_get_supported_formats(
  state: &mut OpState,
  surface_rid: ResourceId,
  adapter_rid: ResourceId,
) -> Result<Vec<wgpu_types::TextureFormat>, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;
  let adapter_resource = state
    .resource_table
    .get::<super::WebGpuAdapter>(adapter_rid)?;
  let adapter = adapter_resource.0;

  gfx_select!(adapter =>
    instance.surface_get_supported_formats(surface, adapter)
  )
  .map_err(AnyError::from)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpuPresentMode {
  AutoVsync = 0,
  AutoNoVsync = 1,
  Fifo = 2,
  FifoRelaxed = 3,
  Immediate = 4,
  Mailbox = 5,
}

impl From<GpuPresentMode> for wgpu_types::PresentMode {
  fn from(value: GpuPresentMode) -> Self {
    match value {
      GpuPresentMode::AutoVsync => wgpu_types::PresentMode::AutoVsync,
      GpuPresentMode::AutoNoVsync => wgpu_types::PresentMode::AutoNoVsync,
      GpuPresentMode::Fifo => wgpu_types::PresentMode::Fifo,
      GpuPresentMode::FifoRelaxed => wgpu_types::PresentMode::FifoRelaxed,
      GpuPresentMode::Immediate => wgpu_types::PresentMode::Immediate,
      GpuPresentMode::Mailbox => wgpu_types::PresentMode::Mailbox,
    }
  }
}

impl From<wgpu_types::PresentMode> for GpuPresentMode {
  fn from(value: wgpu_types::PresentMode) -> Self {
    match value {
      wgpu_types::PresentMode::AutoVsync => GpuPresentMode::AutoVsync,
      wgpu_types::PresentMode::AutoNoVsync => GpuPresentMode::AutoNoVsync,
      wgpu_types::PresentMode::Fifo => GpuPresentMode::Fifo,
      wgpu_types::PresentMode::FifoRelaxed => GpuPresentMode::FifoRelaxed,
      wgpu_types::PresentMode::Immediate => GpuPresentMode::Immediate,
      wgpu_types::PresentMode::Mailbox => GpuPresentMode::Mailbox,
    }
  }
}

#[op]
pub fn op_webgpu_surface_get_supported_modes(
  state: &mut OpState,
  surface_rid: ResourceId,
  adapter_rid: ResourceId,
) -> Result<Vec<GpuPresentMode>, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;
  let adapter_resource = state
    .resource_table
    .get::<super::WebGpuAdapter>(adapter_rid)?;
  let adapter = adapter_resource.0;

  match gfx_select!(adapter =>
    instance.surface_get_supported_modes(surface, adapter)
  ) {
    Ok(modes) => Ok(modes.iter().map(|&mode| mode.into()).collect()),
    Err(err) => Err(err.into()),
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceConfigureArgs {
  surface_rid: ResourceId,
  device_rid: ResourceId,
  format: wgpu_types::TextureFormat,
  usage: u32,
  size: wgpu_types::Extent3d,
  present_mode: GpuPresentMode,
}

#[op]
pub fn op_webgpu_surface_configure(
  state: &mut OpState,
  args: SurfaceConfigureArgs,
) -> Result<(), AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(args.device_rid)?;
  let device = device_resource.0;
  let surface_resource = state
    .resource_table
    .get::<WebGpuSurface>(args.surface_rid)?;
  let surface = surface_resource.0;

  let conf = wgpu_types::SurfaceConfiguration {
    usage: wgpu_types::TextureUsages::from_bits_truncate(args.usage),
    format: args.format,
    width: args.size.width,
    height: args.size.height,
    present_mode: args.present_mode.into(),
  };

  match gfx_select!(device =>
    instance.surface_configure(surface, device, &conf)
  ) {
    None => Ok(()),
    Some(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_get_current_texture(
  state: &mut OpState,
  device_rid: ResourceId,
  surface_rid: ResourceId,
) -> Result<ResourceId, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(device_rid)?;
  let device = device_resource.0;
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  match gfx_select!(device =>
    instance.surface_get_current_texture(surface, PhantomData)
  ) {
    Ok(output) => {
      check_status(output.status)?;
      let texture_id = output.texture_id.unwrap();
      let texture_rid = state
        .resource_table
        .add(crate::texture::WebGpuTexture(texture_id));
      Ok(texture_rid)
    }
    Err(err) => Err(err.into()),
  }
}

#[op]
pub fn op_webgpu_surface_present(
  state: &mut OpState,
  device_rid: ResourceId,
  surface_rid: ResourceId,
) -> Result<(), AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(device_rid)?;
  let device = device_resource.0;
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  match gfx_select!(device => instance.surface_present(surface)) {
    Ok(status) => check_status(status),
    Err(err) => Err(err.into()),
  }
}

fn check_status(status: wgpu_types::SurfaceStatus) -> Result<(), AnyError> {
  use wgpu_types::SurfaceStatus::*;
  let msg = match status {
    Good | Suboptimal => return Ok(()),
    Timeout => "Unable to get the next frame, timed out.",
    Outdated => "The surface under the swap chain has changed.",
    Lost => "The surface under the swap chain is lost.",
  };
  Err(AnyError::msg(msg))
}
