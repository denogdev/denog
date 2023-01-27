// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use super::WebGpuResult;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use serde::Deserialize;
use std::borrow::Cow;
use std::marker::PhantomData;
use wgpu_types::SurfaceStatus;

pub struct WebGpuSurface(pub wgpu_core::id::SurfaceId);
impl Resource for WebGpuSurface {
  fn name(&self) -> Cow<str> {
    "webGPUSurface".into()
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceConfigureArgs {
  surface_rid: ResourceId,
  device_rid: ResourceId,
  format: wgpu_types::TextureFormat,
  usage: u32,
  width: u32,
  height: u32,
}

#[op]
pub fn op_webgpu_surface_configure(
  state: &mut OpState,
  args: SurfaceConfigureArgs,
) -> Result<WebGpuResult, AnyError> {
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
    width: args.width,
    height: args.height,
    present_mode: wgpu_types::PresentMode::Fifo,
  };

  let err =
    gfx_select!(device => instance.surface_configure(surface, device, &conf));

  Ok(WebGpuResult::maybe_err(err))
}

#[op]
pub fn op_webgpu_surface_get_current_texture(
  state: &mut OpState,
  device_rid: ResourceId,
  surface_rid: ResourceId,
) -> Result<WebGpuResult, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(device_rid)?;
  let device = device_resource.0;
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  let output = gfx_select!(device => instance.surface_get_current_texture(surface, PhantomData))?;

  match output.status {
    SurfaceStatus::Good | SurfaceStatus::Suboptimal => {
      let id = output.texture_id.unwrap();
      let rid = state.resource_table.add(crate::texture::WebGpuTexture(id));
      Ok(WebGpuResult::rid(rid))
    }
    _ => Err(AnyError::msg("Invalid Surface Status")),
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

  let _ = gfx_select!(device => instance.surface_present(surface))?;

  Ok(())
}
