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

  // https://docs.rs/winit/0.28.1/winit/event/enum.ElementState.html
  export type WSIButtonState =
    | "pressed"
    | "released";

  // https://docs.rs/winit/0.28.1/winit/window/struct.WindowBuilder.html
  export interface WSICreateWindowOptions {
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

  // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html
  export type WSIEvent =
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.Resumed
      type: "app-resumed";
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.Suspended
      type: "app-suspended";
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.CloseRequested
      type: "close-requested";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.CursorEntered
      type: "cursor-entered";
      window: WSIWindow;
      deviceId: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.CursorLeft
      type: "cursor-left";
      window: WSIWindow;
      deviceId: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.CursorMoved
      type: "cursor-moved";
      window: WSIWindow;
      deviceId: number;
      position: [number, number];
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Added
      type: "device-added";
      deviceId: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Motion
      type: "device-axis";
      deviceId: number;
      axisId: number;
      value: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Button
      type: "device-button";
      deviceId: number;
      button: number;
      state: WSIButtonState;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Text
      type: "device-char";
      deviceId: number;
      codePoint: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Key
      type: "device-key";
      deviceId: number;
      scanCode: number;
      keyCode?: WSIKeyCode;
      state: WSIButtonState;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.Removed
      type: "device-removed";
      deviceId: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.MouseWheel
      type: "device-scroll";
      deviceId: number;
      delta: WSIScrollDelta;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.DroppedFile
      type: "file-dropped";
      window: WSIWindow;
      path: string;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.HoveredFile
      type: "file-hovered";
      window: WSIWindow;
      path: string;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.HoveredFileCancelled
      type: "file-left";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Ime.html#variant.Commit
      type: "ime-commit";
      window: WSIWindow;
      string: string;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Ime.html#variant.Disabled
      type: "ime-disabled";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Ime.html#variant.Enabled
      type: "ime-enabled";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Ime.html#variant.Preedit
      type: "ime-preedit";
      window: WSIWindow;
      string: string;
      cursorRange?: [number, number];
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.AxisMotion
      type: "input-axis";
      window: WSIWindow;
      deviceId: number;
      axisId: number;
      value: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.ReceivedCharacter
      type: "input-char";
      window: WSIWindow;
      codePoint: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.KeyboardInput
      type: "input-key";
      window: WSIWindow;
      deviceId: number;
      scanCode: number;
      keyCode?: WSIKeyCode;
      state: WSIButtonState;
      isSynthetic: boolean;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.Touch
      type: "input-touch";
      window: WSIWindow;
      deviceId: number;
      location: [number, number];
      touchPhase: WSITouchPhase;
      touchForce?: WSITouchForce;
      fingerId: bigint;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.MainEventsCleared
      type: "main-events-cleared";
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.ModifiersChanged
      type: "modifiers-changed";
      window: WSIWindow;
      modifiers: WSIModifierKeys;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.MouseInput
      type: "mouse-button";
      window: WSIWindow;
      deviceId: number;
      button: WSIMouseButton;
      state: WSIButtonState;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.MouseMotion
      type: "mouse-motion";
      deviceId: number;
      delta: WSIMouseDelta;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.MouseWheel
      type: "mouse-scroll";
      window: WSIWindow;
      deviceId: number;
      delta: WSIScrollDelta;
      touchPhase: WSITouchPhase;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.NewEvents
      type: "new-events";
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.RedrawEventsCleared
      type: "redraw-events-cleared";
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.Event.html#variant.RedrawRequested
      type: "redraw-requested";
      window: WSIWindow;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.ScaleFactorChanged
      type: "scale-factor-changed";
      window: WSIWindow;
      scaleFactor: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.SmartMagnify
      type: "smart-magnify";
      window: WSIWindow;
      deviceId: number;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.TouchpadMagnify
      type: "touchpad-magnify";
      window: WSIWindow;
      deviceId: number;
      delta: number;
      touchPhase: WSITouchPhase;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.TouchpadPressure
      type: "touchpad-pressure";
      window: WSIWindow;
      deviceId: number;
      pressure: number;
      clickLevel: bigint;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.TouchpadRotate
      type: "touchpad-rotate";
      window: WSIWindow;
      deviceId: number;
      delta: number;
      touchPhase: WSITouchPhase;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.Focused
      type: "window-focus";
      window: WSIWindow;
      hasFocus: boolean;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.Moved
      type: "window-moved";
      window: WSIWindow;
      position: [number, number];
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.Occluded
      type: "window-occlusion";
      window: WSIWindow;
      isOccluded: boolean;
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.Resized
      type: "window-resized";
      window: WSIWindow;
      innerSize: [number, number];
    }
    | {
      // https://docs.rs/winit/0.28.1/winit/event/enum.WindowEvent.html#variant.ThemeChanged
      type: "window-theme-changed";
      window: WSIWindow;
      theme: WSIWindowTheme;
    };

  // https://docs.rs/winit/0.28.1/winit/event/enum.VirtualKeyCode.html
  export type WSIKeyCode =
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
    | "left-ctrl"
    | "left-shift"
    | "left-gui"
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
    | "right-ctrl"
    | "right-shift"
    | "right-gui"
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

  // https://docs.rs/winit/0.28.1/winit/event/struct.ModifiersState.html
  export type WSIModifierKeys = number;
  export class WSIModifierKey {
    static SHIFT: 0o0004;
    static CTRL: 0o0040;
    static ALT: 0o0400;
    static GUI: 0o4000;
  }

  // https://docs.rs/winit/0.28.1/winit/event/enum.MouseButton.html
  export type WSIMouseButton =
    | "left"
    | "right"
    | "middle"
    | number;

  // https://docs.rs/winit/0.28.1/winit/event/enum.DeviceEvent.html#variant.MouseMotion
  export type WSIMouseDelta = {
    x: number;
    y: number;
  };

  // https://docs.rs/winit/0.28.1/winit/event/enum.MouseScrollDelta.html
  export type WSIScrollDelta = {
    type: "line-delta" | "pixel-delta";
    x: number;
    y: number;
  };

  // https://docs.rs/winit/0.28.1/winit/event/enum.Force.html
  export type WSITouchForce =
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

  // https://docs.rs/winit/0.28.1/winit/event/enum.TouchPhase.html
  export type WSITouchPhase =
    | "started"
    | "moved"
    | "ended"
    | "cancelled";

  // https://docs.rs/winit/0.28.1/winit/window/struct.Window.html
  export class WSIWindow {
    setContentProtected(contentProtected?: boolean): void;
    isDecorated(): boolean;
    setDecorated(decorated?: boolean): void;
    getEnabledButtons(): WSIWindowButtons;
    setEnabledButtons(buttons: WSIWindowButtons): void;
    hasFocus(): boolean;
    focus(): void;
    isFullscreen(): boolean;
    setFullscreen(fullscreen?: boolean): void;
    getGPUSurface(): GPUSurface;
    getInnerPosition(): [number, number] | null;
    getOuterPosition(): [number, number] | null;
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
    requestRedraw(): void;
    destroy(): void;
  }

  // https://docs.rs/winit/latest/winit/window/struct.WindowButtons.html
  export type WSIWindowButtons = number;
  export class WSIWindowButton {
    static CLOSE: 0b001;
    static MINIMIZE: 0b010;
    static MAXIMIZE: 0b100;
  }

  // https://docs.rs/winit/latest/winit/window/enum.WindowLevel.html
  export type WSIWindowLevel =
    | "always-on-bottom"
    | "normal"
    | "always-on-top";

  // https://docs.rs/winit/0.28.1/winit/window/enum.Theme.html
  export type WSIWindowTheme =
    | "light"
    | "dark";
}
