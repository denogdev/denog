// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

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

  let result = gfx_select!(adapter => instance.surface_get_supported_formats(
    surface, adapter));

  result.map_err(|err| AnyError::from(err))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceConfigureArgs {
  surface_rid: ResourceId,
  device_rid: ResourceId,
  format: wgpu_types::TextureFormat,
  usage: u32,
  size: wgpu_types::Extent3d,
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
    width: args.size.width,
    height: args.size.height,
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
) -> Result<(Option<ResourceId>, Option<String>), AnyError> {
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
    Ok(output) => {
      let texture_rid = output.texture_id.map(|texture_id| {
        state
          .resource_table
          .add(crate::texture::WebGpuTexture(texture_id))
      });
      let err_msg = maybe_err_msg_from_status(output.status);
      Ok((texture_rid, err_msg))
    }
    Err(err) => Ok((None, Some(err.to_string()))),
  }
}

#[op]
pub fn op_webgpu_surface_present(
  state: &mut OpState,
  device_rid: ResourceId,
  surface_rid: ResourceId,
) -> Result<Option<String>, AnyError> {
  let instance = state.borrow::<super::Instance>();
  let device_resource = state
    .resource_table
    .get::<super::WebGpuDevice>(device_rid)?;
  let device = device_resource.0;
  let surface_resource =
    state.resource_table.get::<WebGpuSurface>(surface_rid)?;
  let surface = surface_resource.0;

  match gfx_select!(device => instance.surface_present(surface)) {
    Ok(status) => Ok(maybe_err_msg_from_status(status)),
    Err(err) => Ok(Some(err.to_string())),
  }
}

fn maybe_err_msg_from_status(
  status: wgpu_types::SurfaceStatus,
) -> Option<String> {
  use wgpu_types::SurfaceStatus::*;
  let msg = match status {
    Good | Suboptimal => return None,
    Timeout => "Unable to get the next frame, timed out.",
    Outdated => "The surface under the swap chain has changed.",
    Lost => "The surface under the swap chain is lost.",
  };
  Some(msg.to_string())
}
