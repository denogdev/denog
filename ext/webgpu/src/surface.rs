// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

use deno_core::Resource;
use std::borrow::Cow;

pub struct WebGpuSurface(pub wgpu_core::id::SurfaceId);
impl Resource for WebGpuSurface {
  fn name(&self) -> Cow<str> {
    "webGPUSurface".into()
  }
}
