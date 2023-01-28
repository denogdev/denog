// Copyright 2023 Jo Bates. All rights reserved. MIT license.

// Create window.
const window = Deno.wsi.createWindow({
  title: "Example",
  visible: false,
});

// Get WebGPU surface.
const surface = window.getGPUSurface();

// Choose WebGPU adapter.
const adapter = await navigator.gpu.requestAdapter({
  compatibleSurface: surface,
});

// Create WebGPU device.
const device = await adapter.requestDevice();

// Configure WebGPU surface.
configureSurface(window.getInnerSize());
function configureSurface([width, height]) {
  surface.configure({
    device,
    format: "bgra8unorm-srgb",
    width,
    height,
  });
}

// Show window.
window.setVisible();

// Event loop.
while (true) {
  const event = await Deno.wsi.nextEvent();
  switch (event.type) {
    // Reconfigure WebGPU surface when the window size changes.
    case "window-resized": {
      const { innerSize } = event;
      configureSurface(innerSize);
      window.requestRedraw();
      break;
    }

    // Redraw.
    case "redraw-requested": {
      const commandEncoder = device.createCommandEncoder();
      const renderPassEncoder = commandEncoder.beginRenderPass({
        colorAttachments: [
          {
            view: surface.getCurrentTexture().createView(),
            clearValue: { r: 0.0, g: 0.2, b: 0.4, a: 1.0 },
            loadOp: "clear",
            storeOp: "store",
          },
        ],
      });
      renderPassEncoder.end();
      device.queue.submit([commandEncoder.finish()]);
      surface.present();
      break;
    }

    // Close.
    case "close-requested": {
      Deno.exit(0);
    }
  }
}
