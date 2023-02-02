// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// Copyright 2023 Jo Bates. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="deno.ns" />

declare namespace Deno {
  export {}; // stop default export type behavior

  export type WSIButtonState =
    | "pressed"
    | "released";

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

  export type WSIMouseButton =
    | "left"
    | "right"
    | "middle"
    | number;

  export type WSIMouseDelta = {
    x: number;
    y: number;
  };

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

  export type WSITouchPhase =
    | "started"
    | "moved"
    | "ended"
    | "cancelled";

  export type WSIScrollDelta = {
    type: "line-delta" | "pixel-delta";
    x: number;
    y: number;
  };

  export type WSIWindowTheme =
    | "light"
    | "dark";

  export type WSIEvent =
    | {
      type: "app-resumed";
    }
    | {
      type: "app-suspended";
    }
    | {
      type: "axis-input";
      window: WSIWindow;
      deviceId: number;
      axisId: number;
      value: number;
    }
    | {
      type: "char-input";
      window: WSIWindow;
      codePoint: number;
    }
    | {
      type: "close-requested";
      window: WSIWindow;
    }
    | {
      type: "cursor-entered";
      window: WSIWindow;
      deviceId: number;
    }
    | {
      type: "cursor-left";
      window: WSIWindow;
      deviceId: number;
    }
    | {
      type: "cursor-moved";
      window: WSIWindow;
      deviceId: number;
      position: [number, number];
    }
    | {
      type: "device-added";
      deviceId: number;
    }
    | {
      type: "device-axis";
      deviceId: number;
      value: number;
    }
    | {
      type: "device-button";
      deviceId: number;
      button: number;
      state: WSIButtonState;
    }
    | {
      type: "device-char";
      deviceId: number;
      codePoint: number;
    }
    | {
      type: "device-key";
      deviceId: number;
      scanCode: number;
      keyCode?: WSIKeyCode;
      state: WSIButtonState;
    }
    | {
      type: "device-removed";
      deviceId: number;
    }
    | {
      type: "device-scroll";
      deviceId: number;
      delta: WSIScrollDelta;
    }
    | {
      type: "dropped-file";
      window: WSIWindow;
      path: string;
    }
    | {
      type: "hovered-file";
      window: WSIWindow;
      path: string;
    }
    | {
      type: "hovered-file-cancelled";
      window: WSIWindow;
    }
    | {
      type: "ime-commit";
      window: WSIWindow;
      string: string;
    }
    | {
      type: "ime-disabled";
      window: WSIWindow;
    }
    | {
      type: "ime-enabled";
      window: WSIWindow;
    }
    | {
      type: "ime-preedit";
      window: WSIWindow;
      string: string;
      cursorRange?: [number, number];
    }
    | {
      type: "key-input";
      window: WSIWindow;
      deviceId: number;
      scanCode: number;
      keyCode?: WSIKeyCode;
      state: WSIButtonState;
      isSynthetic: boolean;
    }
    | {
      type: "main-events-cleared";
    }
    | {
      type: "modifiers-changed";
      window: WSIWindow;
      modifiers: WSIModifierKeys;
    }
    | {
      type: "mouse-button";
      window: WSIWindow;
      deviceId: number;
      button: WSIMouseButton;
      state: WSIButtonState;
    }
    | {
      type: "mouse-motion";
      deviceId: number;
      delta: WSIMouseDelta;
    }
    | {
      type: "mouse-scroll";
      window: WSIWindow;
      deviceId: number;
      delta: WSIScrollDelta;
      touchPhase: WSITouchPhase;
    }
    | {
      type: "new-events";
    }
    | {
      type: "redraw-events-cleared";
    }
    | {
      type: "redraw-requested";
      window: WSIWindow;
    }
    | {
      type: "scale-factor-changed";
      window: WSIWindow;
      scaleFactor: number;
    }
    | {
      type: "touch-input";
      window: WSIWindow;
      deviceId: number;
      location: [number, number];
      touchPhase: WSITouchPhase;
      touchForce?: WSITouchForce;
      fingerId: bigint;
    }
    | {
      type: "touchpad-pressure";
      window: WSIWindow;
      deviceId: number;
      pressure: number;
      clickLevel: bigint;
    }
    | {
      type: "window-focused";
      window: WSIWindow;
      isFocused: boolean;
    }
    | {
      type: "window-moved";
      window: WSIWindow;
      position: [number, number];
    }
    | {
      type: "window-occluded";
      window: WSIWindow;
      isOccluded: boolean;
    }
    | {
      type: "window-resized";
      window: WSIWindow;
      innerSize: [number, number];
    }
    | {
      type: "window-theme-changed";
      window: WSIWindow;
      theme: WSIWindowTheme;
    };

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

  export const wsi: WSI;
  export class WSI {
    nextEvent(): Promise<WSIEvent>;
    createWindow(options?: WSICreateWindowOptions): WSIWindow;
  }

  export type WSIModifierKeys = number;
  export class WSIModifierKey {
    static SHIFT: 0o0004;
    static CTRL: 0o0040;
    static ALT: 0o0400;
    static GUI: 0o4000;
  }

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
