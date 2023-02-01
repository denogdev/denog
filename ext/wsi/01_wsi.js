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

  function assertWindow(window, { prefix, context }) {
    const wid = window[_wid];
    if (wid === undefined) {
      throw new DOMException(
        `${prefix}: ${context} references an invalid or destroyed window.`,
        "OperationError",
      );
    }
    return wid;
  }

  class WSIModifiersState {
    constructor() {
      webidl.illegalConstructor();
    }

    static get SHIFT() {
      return 0o0004;
    }
    static get CTRL() {
      return 0o0040;
    }
    static get ALT() {
      return 0o0400;
    }
    static get LOGO() {
      return 0o4000;
    }
  }

  class WSIWindow {
    [_wid] = null;
    [_gpuSurface] = null;

    constructor() {
      webidl.illegalConstructor();
    }

    destroy() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'destroy' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      if (this[_gpuSurface]) {
        webgpu.destroyGPUSurface(this[_gpuSurface]);
        this[_gpuSurface] = null;
      }

      ops.op_wsi_destroy_window(wid);
      windows.delete(wid);
      this[_wid] = null;
    }

    getGPUSurface() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getGPUSurface' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      if (this[_gpuSurface]) {
        return this[_gpuSurface];
      } else {
        const rid = ops.op_wsi_create_webgpu_surface(wid);
        return this[_gpuSurface] = webgpu.createGPUSurface(rid);
      }
    }

    getScaleFactor() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getScaleFactor' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_scale_factor(wid);
    }

    requestRedraw() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'requestRedraw' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_request_redraw(wid);
    }

    getInnerPosition() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getInnerPosition' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_inner_position(wid);
    }

    getOuterPosition() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getOuterPosition' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_outer_position(wid);
    }

    setOuterPosition(positionOrX, y) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setOuterPosition' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      webidl.requiredArguments(arguments.length, 1, { prefix });
      const position = convertPosition(prefix, positionOrX, y);

      return ops.op_wsi_window_set_outer_position(wid, position);
    }

    getInnerSize() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getInnerSize' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_inner_size(wid);
    }

    setInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setInnerSize' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height);

      return ops.op_wsi_window_set_inner_size(wid, size);
    }

    getOuterSize() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'getOuterSize' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_outer_size(wid);
    }

    setMinInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMinInnerSize' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height, true);

      return ops.op_wsi_window_set_min_inner_size(wid, size);
    }

    setMaxInnerSize(sizeOrWidth, height) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMaxInnerSize' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      webidl.requiredArguments(arguments.length, 1, { prefix });
      const size = convertSize(prefix, sizeOrWidth, height, true);

      return ops.op_wsi_window_set_max_inner_size(wid, size);
    }

    setTitle(title) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setTitle' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      webidl.requiredArguments(arguments.length, 1, { prefix });
      title = webidl.converters["DOMString"](title, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_title(wid, title);
    }

    setVisible(visible = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setVisible' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      visible = webidl.converters["boolean"](visible, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_visible(wid, visible);
    }

    isVisible() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'isVisible' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_is_visible(wid);
    }

    setResizable(resizable = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setResizable' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      resizable = webidl.converters["boolean"](resizable, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_resizable(wid, resizable);
    }

    isResizable() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'isResizable' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_is_resizable(wid);
    }

    setMinimized(minimized = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMinimized' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      minimized = webidl.converters["boolean"](minimized, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_minimized(wid, minimized);
    }

    setMaximized(maximized = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setMaximized' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      maximized = webidl.converters["boolean"](maximized, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_maximized(wid, maximized);
    }

    isMaximized() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'isMaximized' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_is_maximized(wid);
    }

    setFullscreen(fullscreen = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setFullscreen' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      fullscreen = webidl.converters["boolean"](fullscreen, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_fullscreen(wid, fullscreen);
    }

    isFullscreen() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'isFullscreen' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_is_fullscreen(wid);
    }

    setDecorated(decorated = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setDecorated' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      decorated = webidl.converters["boolean"](decorated, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_decorated(wid, decorated);
    }

    isDecorated() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'isDecorated' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_window_is_decorated(wid);
    }

    setAlwaysOnTop(alwaysOnTop = true) {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'setAlwaysOnTop' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      alwaysOnTop = webidl.converters["boolean"](alwaysOnTop, {
        prefix,
        context: "Argument 1",
      });

      return ops.op_wsi_window_set_always_on_top(wid, alwaysOnTop);
    }

    focus() {
      webidl.assertBranded(this, WSIWindowPrototype);
      const prefix = "Failed to execute 'focus' on 'WSIWindow'";
      const wid = assertWindow(this, { prefix, context: "this" });

      return ops.op_wsi_focus_window(wid);
    }
  }
  const WSIWindowPrototype = WSIWindow.prototype;

  globalThis.__bootstrap.wsi = {
    wsi: webidl.createBranded(WSI),
    WSI,
    WSIModifiersState,
    WSIWindow,
  };
})(this);
