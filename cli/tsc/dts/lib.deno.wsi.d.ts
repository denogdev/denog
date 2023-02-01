// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="deno.ns" />

declare namespace Deno {
  export {}; // stop default export type behavior

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export interface WSICreateWindowOptions {
    innerSize?: [number, number];
    minInnerSize?: [number, number];
    maxInnerSize?: [number, number];
    position?: [number, number];
    resizable?: boolean;
    title?: string;
    fullscreen?: boolean;
    maximized?: boolean;
    visible?: boolean;
    transparent?: boolean;
    decorated?: boolean;
    alwaysOnTop?: boolean;
  }

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSIElementState = "pressed" | "released";

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSIEvent =
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Motion
      // and
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.AxisMotion
      type: "axis-motion";
      window?: WSIWindow;
      axis: number;
      value: number;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Button
      type: "button";
      button: number;
      state: WSIElementState;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Text
      // and
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.ReceivedCharacter
      type: "character";
      window?: WSIWindow;
      codePoint: number;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.CloseRequested
      type: "close-requested";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.CursorEntered
      type: "cursor-entered";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.CursorLeft
      type: "cursor-left";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.CursorMoved
      type: "cursor-moved";
      window: WSIWindow;
      position: [number, number];
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Added
      type: "device-added";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Removed
      type: "device-removed";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.DroppedFile
      type: "dropped-file";
      window: WSIWindow;
      path: string;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.HoveredFile
      type: "hovered-file";
      window: WSIWindow;
      path: string;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.HoveredFileCancelled
      type: "hovered-file-cancelled";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Ime
      type: "ime";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Key
      // and
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.KeyboardInput
      type: "key";
      window?: WSIWindow;
      scanCode: number;
      state: WSIElementState;
      synthetic?: boolean;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.MainEventsCleared
      type: "main-events-cleared";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.ModifiersChanged
      type: "modifiers-changed";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.MouseInput
      type: "mouse-button";
      window: WSIWindow;
      state: WSIElementState;
      button: WSIMouseButton;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.MouseMotion
      type: "mouse-motion";
      delta: WSIMouseMotionDelta;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.MouseWheel
      // and
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.MouseWheel
      type: "mouse-wheel";
      window?: WSIWindow;
      delta: WSIMouseScrollDelta;
      phase?: WSITouchPhase;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.NewEvents
      type: "new-events";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.RedrawEventsCleared
      type: "redraw-events-cleared";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.RedrawRequested
      type: "redraw-requested";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.Resumed
      type: "resumed";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.ScaleFactorChanged
      type: "scale-factor-changed";
      window: WSIWindow;
      scaleFactor: number;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Event.html#variant.Suspended
      type: "suspended";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.ThemeChanged
      type: "theme-changed";
      window: WSIWindow;
      theme: WSITheme;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Touch
      type: "touch";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.TouchpadPressure
      type: "touchpad-pressure";
      window: WSIWindow;
      pressure: number;
      stage: bigint;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Focused
      type: "window-focused";
      window: WSIWindow;
      focused: boolean;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Moved
      type: "window-moved";
      window: WSIWindow;
      position: [number, number];
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Occluded
      type: "window-occluded";
      window: WSIWindow;
      occluded: number;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Resized
      type: "window-resized";
      window: WSIWindow;
      innerSize: [number, number];
    };

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSIMouseButton = "left" | "right" | "middle" | number;

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export interface WSIMouseMotionDelta {
    x: number;
    y: number;
  }

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export interface WSIMouseScrollDelta {
    type: WSIMouseScrollDeltaType;
    x: number;
    y: number;
  }

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSIMouseScrollDeltaType = "line" | "pixel";

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSITheme = "light" | "dark";

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export type WSITouchPhase = "started" | "moved" | "ended" | "cancelled";

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export const wsi: WSI;

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export class WSI {
    nextEvent(): Promise<WSIEvent>;
    createWindow(options?: WSICreateWindowOptions): WSIWindow;
  }

  /** **UNSTABLE**: New API, yet to be vetted.
   *
   * @category Window System Integration
   */
  export class WSIWindow {
    destroy(): void;
    getGPUSurface(): GPUSurface;
    getScaleFactor(): number;
    requestRedraw(): void;
    getInnerPosition(): [number, number] | null;
    getOuterPosition(): [number, number] | null;
    setOuterPosition(position: [number, number]): void;
    setOuterPosition(x: number, y: number): void;
    getInnerSize(): [number, number];
    setInnerSize(size: [number, number]): void;
    setInnerSize(width: number, height: number): void;
    getOuterSize(): [number, number];
    setMinInnerSize(size: [number, number] | null): void;
    setMinInnerSize(width: number, height: number): void;
    setMaxInnerSize(size: [number, number] | null): void;
    setMaxInnerSize(width: number, height: number): void;
    setTitle(title: string): void;
    setVisible(visible?: boolean): void;
    isVisible(): boolean | null;
    setResizable(resizable?: boolean): void;
    isResizable(): boolean;
    setMinimized(minimized?: boolean): void;
    setMaximized(maximized?: boolean): void;
    isMaximized(): boolean;
    setFullscreen(fullscreen?: boolean): void;
    isFullscreen(): boolean;
    setDecorated(decorated?: boolean): void;
    isDecorated(): boolean;
    setAlwaysOnTop(alwaysOnTop?: boolean): void;
    focus(): void;
  }
}
