// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="deno.ns" />

declare namespace Deno {
  export {}; // stop default export type behavior

  export const wsi: WSI;

  export class WSI {
    nextEvent(): Promise<WSIEvent>;
    createWindow(options?: WSICreateWindowOptions): WSIWindow;
  }

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

  export type WSIElementState = "pressed" | "released";

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
      // https://docs.rs/winit/0.27.5/winit/event/enum.Ime.html#variant.Commit
      type: "ime-commit";
      window: WSIWindow;
      string: string;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Ime.html#variant.Disabled
      type: "ime-disabled";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Ime.html#variant.Enabled
      type: "ime-enabled";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.Ime.html#variant.Preedit
      type: "ime-preedit";
      window: WSIWindow;
      string: string;
      cursorRange?: [number, number];
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.DeviceEvent.html#variant.Key
      // and
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.KeyboardInput
      type: "key";
      window?: WSIWindow;
      scanCode: number;
      state: WSIElementState;
      virtualKeyCode?: WSIVirtualKeyCode;
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
      modifiers: WSIModifiers;
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.MouseInput
      type: "mouse-button";
      window: WSIWindow;
      state: WSIElementState;
      button: "left" | "right" | "middle" | number;
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
      theme: "light" | "dark";
    }
    | {
      // https://docs.rs/winit/0.27.5/winit/event/enum.WindowEvent.html#variant.Touch
      type: "touch";
      window: WSIWindow;
      phase: WSITouchPhase;
      location: [number, number];
      force?: WSIForce;
      id: bigint;
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

  export type WSIForce =
    | {
      type: "calibrated";
      value: number;
      maxValue: number;
      altitudeAngle?: number;
    }
    | {
      type: "normalized";
      value: number;
    };

  export class WSIModifier {
    static SHIFT: 0o0004;
    static CTRL: 0o0040;
    static ALT: 0o0400;
    static LOGO: 0o4000;
  }

  export type WSIModifiers = number;

  export interface WSIMouseMotionDelta {
    x: number;
    y: number;
  }

  export interface WSIMouseScrollDelta {
    type: "line" | "pixel";
    x: number;
    y: number;
  }

  export type WSITouchPhase = "started" | "moved" | "ended" | "cancelled";

  export type WSIVirtualKeyCode =
    | "1"
    | "2"
    | "3"
    | "4"
    | "5"
    | "6"
    | "7"
    | "8"
    | "9"
    | "0"
    | "a"
    | "b"
    | "c"
    | "d"
    | "e"
    | "f"
    | "g"
    | "h"
    | "i"
    | "j"
    | "k"
    | "l"
    | "m"
    | "n"
    | "o"
    | "p"
    | "q"
    | "r"
    | "s"
    | "t"
    | "u"
    | "v"
    | "w"
    | "x"
    | "y"
    | "z"
    | "escape"
    | "f1"
    | "f2"
    | "f3"
    | "f4"
    | "f5"
    | "f6"
    | "f7"
    | "f8"
    | "f9"
    | "f10"
    | "f11"
    | "f12"
    | "f13"
    | "f14"
    | "f15"
    | "f16"
    | "f17"
    | "f18"
    | "f19"
    | "f20"
    | "f21"
    | "f22"
    | "f23"
    | "f24"
    | "snapshot"
    | "scroll"
    | "pause"
    | "insert"
    | "home"
    | "delete"
    | "end"
    | "page-down"
    | "page-up"
    | "left"
    | "up"
    | "right"
    | "down"
    | "back"
    | "return"
    | "space"
    | "compose"
    | "caret"
    | "numlock"
    | "numpad-0"
    | "numpad-1"
    | "numpad-2"
    | "numpad-3"
    | "numpad-4"
    | "numpad-5"
    | "numpad-6"
    | "numpad-7"
    | "numpad-8"
    | "numpad-9"
    | "numpad-add"
    | "numpad-divide"
    | "numpad-decimal"
    | "numpad-comma"
    | "numpad-enter"
    | "numpad-equals"
    | "numpad-multiply"
    | "numpad-subtract"
    | "abnt-c1"
    | "abnt-c2"
    | "apostrophe"
    | "apps"
    | "asterisk"
    | "at"
    | "ax"
    | "backslash"
    | "calculator"
    | "capital"
    | "colon"
    | "comma"
    | "convert"
    | "equals"
    | "grave"
    | "kana"
    | "kanji"
    | "left-alt"
    | "left-bracket"
    | "left-control"
    | "left-shift"
    | "left-win"
    | "mail"
    | "media-select"
    | "media-stop"
    | "minus"
    | "mute"
    | "my-computer"
    | "navigate-forward"
    | "navigate-backward"
    | "next-track"
    | "no-convert"
    | "oem-102"
    | "period"
    | "play-pause"
    | "plus"
    | "power"
    | "prev-track"
    | "right-alt"
    | "right-bracket"
    | "right-control"
    | "right-shift"
    | "right-win"
    | "semicolon"
    | "slash"
    | "sleep"
    | "stop"
    | "sysrq"
    | "tab"
    | "underline"
    | "unlabeled"
    | "volume-down"
    | "volume-up"
    | "wake"
    | "web-back"
    | "web-favorites"
    | "web-forward"
    | "web-home"
    | "web-refresh"
    | "web-search"
    | "web-stop"
    | "yen"
    | "copy"
    | "paste"
    | "cut";

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
