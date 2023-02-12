# Denog

A fork of [Deno](https://github.com/denoland/deno) with built-in window system
integration.

### Install

Shell (Mac, Linux):

```sh
curl -fsSL https://denogdev.github.io/install.sh | sh
```

PowerShell (Windows):

```powershell
irm https://denogdev.github.io/install.ps1 | iex
```

### Getting Started

Try running the
[hello-triangle](https://github.com/denogdev/denogdev.github.io/blob/main/hello-triangle.ts)
example:

```sh
denog run --unstable --wsi https://denogdev.github.io/hello-triangle.ts
```

![A red triangle over a green background.](./examples/hello-triangle/screenshot.png)

Denog is based on Deno and provides most of the same functionality. You can
learn more about Deno from its [manual](https://deno.land/manual).

Deno's complete API reference is available at the runtime
[documentation](https://doc.deno.land). Denog's additions to this API are
documented below.

### Using Visual Studio Code

Denog is compatible with the
[vscode_deno](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno)
extension. Once both Denog and the extension are installed, you can enable the
extension and point it at Denog instead of Deno in your Visual Studio Code
`settings.json`:

```js
{
    "deno.enable": true,
    "deno.unstable": true,
    "deno.path": "C:/Users/<USERNAME>/.deno/bin/denog.exe"
    // Or for macOS: "/Users/<USERNAME>/.deno/bin/denog"
    // Or for Linux: "/home/<USERNAME>/.deno/bin/denog"
}
```

### Window System Integration (WSI)

Denog enhances Deno by adding built-in support for window system integration
(WSI) which can be enabled using the optional `--wsi` flag. It's currently only
available to the `denog run` subcommand and requires the `--unstable` flag as
well. Example:

```sh
denog run --unstable --wsi https://denogdev.github.io/hello-triangle.ts
```

Denog's window system integration uses the Rust
[`winit`](https://docs.rs/winit/0.28.1/winit/) library under the hood and
provides much of the same functionality.

To create a window, use `Deno.wsi.createWindow`:

```ts
createWindow(options?: WSICreateWindowOptions): WSIWindow
```

`WSICreateWindowOptions` provides most of the same options as
[`winit::window::WindowBuilder`](https://docs.rs/winit/0.28.1/winit/window/struct.WindowBuilder.html):

```ts
declare interface WSICreateWindowOptions {
  active?: boolean;
  contentProtected?: boolean;
  decorated?: boolean;
  enabledButtons?: WSIWindowButtons;
  fullscreen?: boolean;
  position?: [number, number];
  innerSize?: [number, number];
  minInnerSize?: [number, number];
  maxInnerSize?: [number, number];
  level?: WSIWindowLevel;
  maximized?: boolean;
  resizable?: boolean;
  resizeIncrements?: [number, number];
  theme?: WSIWindowTheme;
  title?: string;
  transparent?: boolean;
  visible?: boolean;
}
```

You can read and/or modify more properties after a window has been created:

```ts
declare class WSIWindow {
  setContentProtected(contentProtected?: boolean): void;
  setCursorGrabMode(mode: WSICursorGrabMode): void;
  setCursorIcon(icon: WSICursorIcon): void;
  setCursorPosition(position: [number, number]): void;
  setCursorPosition(x: number, y: number): void;
  setCursorVisible(visible?: boolean): void;
  isDecorated(): boolean;
  setDecorated(decorated?: boolean): void;
  getEnabledButtons(): WSIWindowButtons;
  setEnabledButtons(buttons: WSIWindowButtons): void;
  hasFocus(): boolean;
  takeFocus(): void;
  isFullscreen(): boolean;
  setFullscreen(fullscreen?: boolean): void;
  getGPUSurface(): GPUSurface;
  setIMEAllowed(allowed?: boolean): void;
  setIMEPosition(position: [number, number]): void;
  setIMEPosition(x: number, y: number): void;
  setIMEPurpose(purpose: WSIIMEPurpose): void;
  getInnerPosition(): [number, number];
  getOuterPosition(): [number, number];
  setOuterPosition(position: [number, number]): void;
  setOuterPosition(x: number, y: number): void;
  getInnerSize(): [number, number];
  getOuterSize(): [number, number];
  setInnerSize(size: [number, number]): void;
  setInnerSize(width: number, height: number): void;
  setMinInnerSize(size: [number, number] | null): void;
  setMinInnerSize(width: number, height: number): void;
  setMaxInnerSize(size: [number, number] | null): void;
  setMaxInnerSize(width: number, height: number): void;
  setLevel(level: WSIWindowLevel): void;
  isMinimized(): boolean | null;
  setMinimized(minimized?: boolean): void;
  isMaximized(): boolean;
  setMaximized(maximized?: boolean): void;
  isResizable(): boolean;
  setResizable(resizable?: boolean): void;
  getResizeIncrements(): [number, number] | null;
  setResizeIncrements(size: [number, number] | null): void;
  setResizeIncrements(width: number, height: number): void;
  getScaleFactor(): number;
  getTheme(): WSIWindowTheme | null;
  setTheme(theme: WSIWindowTheme | null): void;
  getTitle(): string;
  setTitle(title: string): void;
  setTransparent(transparent?: boolean): void;
  isVisible(): boolean | null;
  setVisible(visible?: boolean): void;
  beginDragMove(): void;
  beginDragResize(direction: WSIResizeDirection): void;
  requestRedraw(): void;
  requestUserAttention(type: WSIUserAttentionType | null): void;
  destroy(): void;
}
```

After creating a window, you generally want to call `Deno.wsi.nextEvent` in an
event loop.

```ts
nextEvent(): Promise<WSIEvent>
```

Example:

```ts
Deno.wsi.createWindow();

eventLoop:
while (true) {
  const event = await Deno.wsi.nextEvent();
  switch (event.type) {
    case "close-requested": {
      break eventLoop;
    }
  }
}
```

`WSIEvent` is a discriminated union. Each `WSIEvent.type` corresponds to a
different event type from
[`winit::event::Event`](https://docs.rs/winit/0.28.1/winit/event/enum.Event.html).
The properties of each event type are listed in
[lib.deno.wsi.d.ts](./cli/tsc/dts/lib.deno.wsi.d.ts) along with links to the
corresponding `winit` events.

### WebGPU Integration

You can render to a `WSIWindow` using the
[WebGPU](https://www.w3.org/TR/webgpu/) API. Unlike standard WebGPU which uses
[`GPUCanvasContext`](https://www.w3.org/TR/webgpu/#canvas-context) to render to
an
[`HTMLCanvasElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement),
Denog provides the non-standard `GPUSurface` class for rendering to a
`WSIWindow` directly. Its interface is much closer to the
[`wgpu::Surface`](https://docs.rs/wgpu/0.15.0/wgpu/struct.Surface.html)
interface from the Rust `wgpu` library that Deno and Denog use under-the-hood.

```ts
declare class GPUSurface {
  getCapabilities(adapter: GPUAdapter): GPUSurfaceCapabilities;
  configure(device: GPUDevice, config: GPUSurfaceConfiguration): void;
  getCurrentTexture(): GPUSurfaceTexture;
}

declare interface GPUSurfaceCapabilities {
  formats: GPUTextureFormat[];
  presentModes: GPUSurfacePresentMode[];
  alphaModes: GPUSurfaceAlphaMode[];
}

declare type GPUSurfacePresentMode =
  | "auto-vsync"
  | "auto-no-vsync"
  | "fifo"
  | "fifo-relaxed"
  | "immediate"
  | "mailbox";

declare type GPUSurfaceAlphaMode =
  | "auto"
  | "opaque"
  | "pre-multiplied"
  | "post-multiplied"
  | "inherit";

declare interface GPUSurfaceConfiguration {
  usage?: GPUTextureUsageFlags;
  format: GPUTextureFormat;
  size: GPUExtent3D;
  presentMode?: GPUSurfacePresentMode;
  alphaMode?: GPUSurfaceAlphaMode;
  viewFormats?: GPUTextureFormat[];
}

declare class GPUSurfaceTexture extends GPUTexture {
  readonly isSuboptimal: boolean;
  present(): void;
}
```

Denog also adds a non-standard `compatibleSurface` property to
[`GPURequestAdapterOptions`](https://www.w3.org/TR/webgpu/#dictdef-gpurequestadapteroptions):

```ts
declare interface GPURequestAdapterOptions {
  powerPreference?: GPUPowerPreference;
  forceFallbackAdapter?: boolean;
  compatibleSurface?: GPUSurface;
}
```

See [examples/hello-triangle](./examples/hello-triangle) for a complete example
of using WSI and WebGPU together.
