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
const format = surface.getSupportedFormats(adapter)[0];

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
const configuration = {
  device,
  format,
  size: window.getInnerSize(),
};
surface.configure(configuration);

// Event loop.
while (true) {
  const event = await Deno.wsi.nextEvent();
  switch (event.type) {
    case "window-resized": {
      configuration.size = event.innerSize;
      surface.configure(configuration);
      window.requestRedraw(); // macOS doesn't do this automatically.
      break;
    }
    case "redraw-requested": {
      const view = surface.getCurrentTexture().createView();
      const commandEncoder = device.createCommandEncoder();

      const renderPass = commandEncoder.beginRenderPass({
        colorAttachments: [{
          view,
          clearValue: { r: 0, g: 1, b: 0, a: 1 },
          loadOp: "clear",
          storeOp: "store",
        }],
      });
      renderPass.setPipeline(pipeline);
      renderPass.draw(3);
      renderPass.end();

      queue.submit([commandEncoder.finish()]);
      surface.present();
      break;
    }
    case "close-requested": {
      Deno.exit(0);
    }
  }
}