// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use super::error::WebGpuError;
use super::WebGpuResult;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use serde::Deserialize;
use std::borrow::Cow;
use std::marker::PhantomData;

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
) -> Result<Option<String>, AnyError> {
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

  Ok(err.map(|err| err.to_string()))
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

  match gfx_select!(
    device => instance.surface_get_current_texture(surface, PhantomData)
  ) {
    Ok(output) => Ok(WebGpuResult {
      rid: output.texture_id.map(|texture_id| {
        state
          .resource_table
          .add(crate::texture::WebGpuTexture(texture_id))
      }),
      err: maybe_err_from_status(output.status),
    }),
    Err(err) => Ok(WebGpuResult::maybe_err(Some(err))),
  }
}

#[op]
pub fn op_webgpu_surface_present(
  state: &mut OpState,
  device_rid: ResourceId,
  surface_rid: ResourceId,
) -> Result<Option<WebGpuError>, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(device_rid)?;
  let device = device_resource.0;
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  match gfx_select!(device => instance.surface_present(surface)) {
    Ok(status) => Ok(maybe_err_from_status(status)),
    Err(err) => Ok(Some(err.into())),
  }
}

fn maybe_err_from_status(
  status: wgpu_types::SurfaceStatus,
) -> Option<WebGpuError> {
  use wgpu_types::SurfaceStatus::*;
  let msg = match status {
    Good | Suboptimal => return None,
    Timeout => "Unable to get the next frame, timed out.",
    Outdated => "The surface under the swap chain has changed.",
    Lost => "The surface under the swap chain is lost.",
  };
  Some(WebGpuError::Validation(msg.to_string()))
}
