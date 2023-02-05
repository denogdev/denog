// Create WSIWindow.
const window = Deno.wsi.createWindow();

// Get GPUSurface.
const surface = window.getGPUSurface();

// Choose GPUAdapter.
const adapter = await navigator.gpu.requestAdapter({
  compatibleSurface: surface,
});
if (!adapter) {
  throw new Error("Failed to find an appropriate adapter");
}

// Create GPUDevice and GPUQueue.
const device = await adapter.requestDevice();
const queue = device.queue;

// Create GPUShaderModule.
const code = `\
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
  let x = f32(i32(in_vertex_index) - 1);
  let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
  return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
`;
const module = device.createShaderModule({
  code,
});

// Create GPUPipelineLayout.
const layout = device.createPipelineLayout({
  bindGroupLayouts: [],
});

// Choose GPUTextureFormat.
const format = surface.getCapabilities(adapter).formats[0];

// Create GPURenderPipeline.
const pipeline = device.createRenderPipeline({
  layout,
  vertex: {
    module,
    entryPoint: "vs_main",
  },
  fragment: {
    module,
    entryPoint: "fs_main",
    targets: [{
      format,
    }],
  },
});

// Configure surface.
const config = {
  format,
  size: window.getInnerSize(),
};
surface.configure(device, config);

// Event loop.
eventLoop:
while (true) {
  const event = await Deno.wsi.nextEvent();
  switch (event.type) {
    case "window-resized": {
      config.size = event.innerSize;
      surface.configure(device, config);
      window.requestRedraw(); // macOS doesn't do this automatically.
      break;
    }
    case "redraw-requested": {
      const texture = surface.getCurrentTexture();
      const view = texture.createView();
      const commandEncoder = device.createCommandEncoder();

      const renderPass = commandEncoder.beginRenderPass({
        colorAttachments: [{
          view,
          clearValue: [0.0, 1.0, 0.0, 1.0],
          loadOp: "clear",
          storeOp: "store",
        }],
      });
      renderPass.setPipeline(pipeline);
      renderPass.draw(3);
      renderPass.end();

      queue.submit([commandEncoder.finish()]);
      texture.present();
      break;
    }
    case "close-requested": {
      break eventLoop;
    }
  }
}
