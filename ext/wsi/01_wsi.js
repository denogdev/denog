// Copyright 2023 Jo Bates. All rights reserved. MIT license.

"use strict";

((globalThis) => {
  const ops = globalThis.Deno.core.ops;
  const webgpu = globalThis.__bootstrap.webgpu;

  const _wid = Symbol("wid");
  const _gpuSurface = Symbol("gpuSurface");

  /** @type {Map<BigInt, WSIWindow>} */
  const windows = new Map();

  /** @returns {WSIEvent} */
  async function nextEvent() {
    const event = await ops.op_wsi_next_event();
    if (event.wid) {
      event.window = windows.get(event.wid);
      delete event.wid;
    }
    return event;
  }

  /**
   * @typedef WSICreateWindowArgs
   * @property {[number, number] | undefined}} innerSize
   * @property {[number, number] | undefined}} minInnerSize
   * @property {[number, number] | undefined}} maxInnerSize
   * @property {[number, number] | undefined}} position
   * @property {bool | undefined}} resizable
   * @property {string | undefined}} title
   * @property {bool | undefined}} fullscreen
   * @property {bool | undefined}} maximized
   * @property {bool | undefined}} visible
   * @property {bool | undefined}} transparent
   * @property {bool | undefined}} decorated
   * @property {bool | undefined}} alwaysOnTop
   */

  /** @param {WSICreateWindowArgs | undefined} args */
  /** @returns {WSIWindow} */
  function createWindow(args) {
    const wid = ops.op_wsi_create_window(args);
    const newWindow = new WSIWindow();
    windows.set(wid, newWindow);
    newWindow[_wid] = wid;
    return newWindow;
  }

  class WSIWindow {
    /** @type {BigInt | undefined} */
    [_wid];

    /** @type {number | null} */
    [_gpuSurface] = null;

    /**
     * @returns {GPUSurface}
     */
    getGPUSurface() {
      const surface = this[_gpuSurface];
      if (surface) {
        return surface;
      } else {
        const rid = ops.op_wsi_create_webgpu_surface(this[_wid]);
        return this[_gpuSurface] = webgpu.createGPUSurface(rid);
      }
    }

    /**
     * @returns {number}
     */
    getScaleFactor() {
      return ops.op_wsi_window_scale_factor(this[_wid]);
    }

    /**
     * @returns {void}
     */
    requestRedraw() {
      return ops.op_wsi_window_request_redraw(this[_wid]);
    }

    /**
     * @returns {[number, number] | null}
     */
    getInnerPosition() {
      return ops.op_wsi_window_inner_position(this[_wid]);
    }

    /**
     * @returns {[number, number] | null}
     */
    getOuterPosition() {
      return ops.op_wsi_window_outer_position(this[_wid]);
    }

    /**
     * @param {[number, number]} position
     * @returns {void}
     *//**
     * @param {number} x
     * @param {number} y
     * @returns {void}
     */
    setOuterPosition(positionOrX, y) {
      const position = (y === undefined) ? positionOrX : [positionOrX, y];
      return ops.op_wsi_window_set_outer_position(this[_wid], position);
    }

    /**
     * @returns {[number, number]}
     */
    getInnerSize() {
      return ops.op_wsi_window_inner_size(this[_wid]);
    }

    /**
     * @param {[number, number]} size
     * @returns {void}
     *//**
     * @param {number} width
     * @param {number} height
     * @returns {void}
     */
    setInnerSize(sizeOrWidth, height) {
      const size = (height === undefined) ? sizeOrWidth : [sizeOrWidth, height];
      return ops.op_wsi_window_set_inner_size(this[_wid], size);
    }

    /**
     * @returns {[number, number]}
     */
    getOuterSize() {
      return ops.op_wsi_window_outer_size(this[_wid]);
    }

    /**
     * @param {[number, number] | null} size
     * @returns {void}
     *//**
     * @param {number} width
     * @param {number} height
     * @returns {void}
     */
    setMinInnerSize(sizeOrWidth, height) {
      const size = (height === undefined) ? sizeOrWidth : [sizeOrWidth, height];
      return ops.op_wsi_window_set_min_inner_size(this[_wid], size);
    }

    /**
     * @param {[number, number] | null} size
     * @returns {void}
     *//**
     * @param {number} width
     * @param {number} height
     * @returns {void}
     */
    setMaxInnerSize(sizeOrWidth, height) {
      const size = (height === undefined) ? sizeOrWidth : [sizeOrWidth, height];
      return ops.op_wsi_window_set_max_inner_size(this[_wid], size);
    }

    /**
     * @param {string} title
     * @returns {void}
     */
    setTitle(title) {
      return ops.op_wsi_window_set_title(this[_wid], title);
    }

    /**
     * @param {bool} visible
     * @returns {void}
     */
    setVisible(visible = true) {
      return ops.op_wsi_window_set_visible(this[_wid], visible);
    }

    /**
     * @returns {bool | null}
     */
    isVisible() {
      return ops.op_wsi_window_is_visible(this[_wid]);
    }

    /**
     * @param {bool} resizable
     * @returns {void}
     */
    setResizable(resizable = true) {
      return ops.op_wsi_window_set_resizable(this[_wid], resizable);
    }

    /**
     * @returns {bool}
     */
    isResizable() {
      return ops.op_wsi_window_is_resizable(this[_wid]);
    }

    /**
     * @param {bool} minimized
     * @returns {void}
     */
    setMinimized(minimized = true) {
      return ops.op_wsi_window_set_minimized(this[_wid], minimized);
    }

    /**
     * @param {bool} maximized
     * @returns {void}
     */
    setMaximized(maximized = true) {
      return ops.op_wsi_window_set_maximized(this[_wid], maximized);
    }

    /**
     * @returns {bool}
     */
    isMaximized() {
      return ops.op_wsi_window_is_maximized(this[_wid]);
    }

    /**
     * @param {bool} fullscreen
     * @returns {void}
     */
    setFullscreen(fullscreen = true) {
      return ops.op_wsi_window_set_fullscreen(this[_wid], fullscreen);
    }

    /**
     * @returns {bool}
     */
    isFullscreen() {
      return ops.op_wsi_window_is_fullscreen(this[_wid]);
    }

    /**
     * @param {bool} decorated
     * @returns {void}
     */
    setDecorated(decorated = true) {
      return ops.op_wsi_window_set_decorated(this[_wid], decorated);
    }

    /**
     * @returns {bool}
     */
    isDecorated() {
      return ops.op_wsi_window_is_decorated(this[_wid]);
    }

    /**
     * @param {bool} alwaysOnTop
     * @returns {void}
     */
    setAlwaysOnTop(alwaysOnTop = true) {
      return ops.op_wsi_window_set_always_on_top(this[_wid], alwaysOnTop);
    }

    /**
     * @returns {void}
     */
    focus() {
      return ops.op_wsi_focus_window(this[_wid]);
    }
  }

  globalThis.__bootstrap.wsi = {
    wsi: {
      nextEvent,
      createWindow,
    },
    WSIWindow,
  };
})(this);
