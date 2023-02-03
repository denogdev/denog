# Denox

A fork of [Deno](https://github.com/denoland/deno) with built-in window system integration.

### Getting Started

Try running the [hello-triangle](./examples/hello-triangle/main.ts) example:

```sh
denox run --unstable --wsi https://raw.githubusercontent.com/denoxdev/denox/v0.3.0/examples/hello-triangle/main.ts
```

![A red triangle over a green background.](./examples/hello-triangle/screenshot.png)

Or a minimal example without any graphics:

```ts
Deno.wsi.createWindow();

while (true) {
  const event = await Deno.wsi.nextEvent();
  switch (event.type) {
    case "close-requested": {
      Deno.exit(0);
    }
  }
}
```
