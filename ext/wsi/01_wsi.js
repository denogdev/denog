// Copyright 2023 Jo Bates. All rights reserved. MIT license.

"use strict";

((globalThis) => {
  const ops = globalThis.Deno.core.ops;
  const webgpu = globalThis.__bootstrap.webgpu;
  const webidl = globalThis.__bootstrap.webidl;

  const _wid = Symbol("wid");
  const _gpuSurface = Symbol("gpuSurface");

  const windows = new Map();

  function convertPosition(prefix, positionOrX, y) {
    if (y === undefined) {
      const position = webidl.converters.WSIPosition(positionOrX, {
        prefix,
        context: "Argument 1",
      });
      checkPosition(prefix, position);
      return position;
    } else {
      const x = webidl.converters["long"](positionOrX, {
        prefix,
        context: "Argument 1",
      });
      y = webidl.converters["long"](y, {
        prefix,
        context: "Argument 2",
      });
      return [x, y];
    }
  }

  function checkPosition(prefix, position) {
    if (position.length != 2) {
      throw new DOMException(
        `${prefix}: position.length must equal 2.`,
        "OperationError",
      );
    }
  }

  function convertSize(prefix, sizeOrWidth, height, optional = false) {
    if (height === undefined) {
      if (optional && sizeOrWidth === null) {
        return null;
      } else {
        const size = webidl.converters.WSISize(sizeOrWidth, {
          prefix,
          context: "Argument 1",
        });
        checkSize(prefix, size);
        return size;
      }
    } else {
      const width = webidl.converters["unsigned long"](sizeOrWidth, {
        prefix,
        context: "Argument 1",
      });
      height = webidl.converters["unsigned long"](height, {
        prefix,
        context: "Argument 2",
      });
      return [width, height];
    }
  }

  function checkSize(prefix, size) {
    if (size.length != 2) {
      throw new DOMException(
        `${prefix}: size.length must equal 2.`,
        "OperationError",
      );
    }
  }

  class WSI {
    [webidl.brand] = webidl.brand;

    constructor() {
      webidl.illegalConstructor();
    }

    async nextEvent() {
      webidl.assertBranded(this, WSIPrototype);
      const event = await ops.op_wsi_next_event();
      if (event.wid) {
        event.window = windows.get(event.wid);
        delete event.wid;
      }
      return event;
    }

    createWindow(options) {
      webidl.assertBranded(this, WSIPrototype);
      const prefix = "Failed to execute 'createWindow' on 'WSI'";
      if (options) {
        options = webidl.converters.WSICreateWindowOptions(options, {
          prefix,
          context: "Argument 1",
        });
        if (options.innerSize) {
          checkSize(prefix, options.innerSize);
        }
        if (options.minInnerSize) {
          checkSize(prefix, options.minInnerSize);
        }
        if (options.maxInnerSize) {
          checkSize(prefix, options.maxInnerSize);
        }
        if (options.position) {
          checkPosition(prefix, options.position);
        }
      }
      const wid = ops.op_wsi_create_window(options);
      const window = webidl.createBranded(WSIWindow);
      windows.set(wid, window);
      window[_wid] = wid;
      return window;
    }
  }
  const WSIPrototype = WSI.prototype;

  class WSIWindow {
    [_wid];
    [_gpuSurface] = null;

    constructor() {
      webidl.illegalConstructor();
    }

    getGPUSurface() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const surface = this[_gpuSurface];
      if (surface) {
        return surface;
      } else {
        const rid = ops.op_wsi_create_webgpu_surface(this[_wid]);
        return this[_gpuSurface] = webgpu.createGPUSurface(rid);
      }
    }

    getScaleFactor() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_scale_factor(this[_wid]);
    }

    requestRedraw() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_request_redraw(this[_wid]);
    }

    getInnerPosition() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_inner_position(this[_wid]);
    }

    getOuterPosition() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_outer_position(this[_wid]);
    }

    setOuterPosition(positionOrX, y) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setOuterPosition' on 'WSIWindow'";
      webidl.requiredArguments(arguments.length, 1, { prefix });
      const position = convertPosition(prefix, positionOrX, y);
      return ops.op_wsi_window_set_outer_position(this[_wid], position);
    }

    getInnerSize() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_inner_size(this[_wid]);
    }

    setInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setInnerSize' on 'WSIWindow'";
      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height);
      return ops.op_wsi_window_set_inner_size(this[_wid], size);
    }

    getOuterSize() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_outer_size(this[_wid]);
    }

    setMinInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMinInnerSize' on 'WSIWindow'";
      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height, true);
      return ops.op_wsi_window_set_min_inner_size(this[_wid], size);
    }

    setMaxInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMaxInnerSize' on 'WSIWindow'";
      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height, true);
      return ops.op_wsi_window_set_max_inner_size(this[_wid], size);
    }

    setTitle(title) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setTitle' on 'WSIWindow'";
      webidl.requiredArguments(arguments.length, 1, { prefix });
      title = webidl.converters["DOMString"](title, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_title(this[_wid], title);
    }

    setVisible(visible = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setVisible' on 'WSIWindow'";
      visible = webidl.converters["boolean"](visible, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_visible(this[_wid], visible);
    }

    isVisible() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_is_visible(this[_wid]);
    }

    setResizable(resizable = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setResizable' on 'WSIWindow'";
      resizable = webidl.converters["boolean"](resizable, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_resizable(this[_wid], resizable);
    }

    isResizable() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_is_resizable(this[_wid]);
    }

    setMinimized(minimized = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMinimized' on 'WSIWindow'";
      minimized = webidl.converters["boolean"](minimized, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_minimized(this[_wid], minimized);
    }

    setMaximized(maximized = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMaximized' on 'WSIWindow'";
      maximized = webidl.converters["boolean"](maximized, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_maximized(this[_wid], maximized);
    }

    isMaximized() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_is_maximized(this[_wid]);
    }

    setFullscreen(fullscreen = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setFullscreen' on 'WSIWindow'";
      fullscreen = webidl.converters["boolean"](fullscreen, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_fullscreen(this[_wid], fullscreen);
    }

    isFullscreen() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_is_fullscreen(this[_wid]);
    }

    setDecorated(decorated = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setDecorated' on 'WSIWindow'";
      decorated = webidl.converters["boolean"](decorated, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_decorated(this[_wid], decorated);
    }

    isDecorated() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_window_is_decorated(this[_wid]);
    }

    setAlwaysOnTop(alwaysOnTop = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setAlwaysOnTop' on 'WSIWindow'";
      alwaysOnTop = webidl.converters["boolean"](alwaysOnTop, {
        prefix,
        context: "Argument 1",
      });
      return ops.op_wsi_window_set_always_on_top(this[_wid], alwaysOnTop);
    }

    focus() {
      webidl.assertBranded(this, WSIWindowPrototype);
      return ops.op_wsi_focus_window(this[_wid]);
    }
  }
  const WSIWindowPrototype = WSIWindow.prototype;

  globalThis.__bootstrap.wsi = {
    wsi: webidl.createBranded(WSI),
    WSI,
    WSIWindow,
  };
})(this);
