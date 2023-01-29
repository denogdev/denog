# Denox

A fork of [Deno](https://github.com/denoland/deno) with built-in window system integration.

### Getting Started

Try running a simple program:

```sh
denox run --unstable --wsi https://raw.githubusercontent.com/denoxdev/denox/v0.1.0/examples/hello-triangle/main.ts
```

Or an even simpler one:

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
