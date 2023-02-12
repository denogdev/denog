// Copyright 2023 Jo Bates. All rights reserved. MIT license.

"use strict";

((globalThis) => {
  const webidl = globalThis.__bootstrap.webidl;

  // ENUM: WSICursorGrabMode
  webidl.converters["WSICursorGrabMode"] = webidl.createEnumConverter(
    "WSICursorGrabMode",
    [
      "none",
      "confined",
      "locked",
    ],
  );

  // ENUM: WSICursorIcon
  webidl.converters["WSICursorIcon"] = webidl.createEnumConverter(
    "WSICursorIcon",
    [
      "default",
      "crosshair",
      "hand",
      "arrow",
      "move",
      "text",
      "wait",
      "help",
      "progress",
      "not-allowed",
      "context-menu",
      "cell",
      "vertical-text",
      "alias",
      "copy",
      "no-drop",
      "grab",
      "grabbing",
      "all-scroll",
      "zoom-in",
      "zoom-out",
      "e-resize",
      "n-resize",
      "ne-resize",
      "nw-resize",
      "s-resize",
      "se-resize",
      "sw-resize",
      "w-resize",
      "ew-resize",
      "ns-resize",
      "nesw-resize",
      "nwse-resize",
      "col-resize",
      "row-resize",
    ],
  );

  // ENUM: WSIIMEPurpose
  webidl.converters["WSIIMEPurpose"] = webidl.createEnumConverter(
    "WSIIMEPurpose",
    [
      "normal",
      "password",
      "terminal",
    ],
  );

  // TYPEDEF: WSIPosition
  webidl.converters["WSIPosition"] = webidl.createSequenceConverter(
    webidl.converters["long"],
  );

  // TYPEDEF: WSISize
  webidl.converters["WSISize"] = webidl.createSequenceConverter(
    webidl.converters["unsigned long"],
  );

  // ENUM: WSIUserAttentionType
  webidl.converters["WSIUserAttentionType"] = webidl.createEnumConverter(
    "WSIUserAttentionType",
    [
      "critical",
      "informational",
    ],
  );

  // ENUM: WSIWindowLevel
  webidl.converters["WSIWindowLevel"] = webidl.createEnumConverter(
    "WSIWindowLevel",
    [
      "always-on-bottom",
      "normal",
      "always-on-top",
    ],
  );

  // ENUM: WSIWindowTheme
  webidl.converters["WSIWindowTheme"] = webidl.createEnumConverter(
    "WSIWindowTheme",
    [
      "light",
      "dark",
    ],
  );

  // DICTIONARY: WSICreateWindowOptions
  const dictMembersWSICreateWindowOptions = [
    {
      key: "active",
      converter: webidl.converters["boolean"],
    },
    {
      key: "contentProtected",
      converter: webidl.converters["boolean"],
    },
    {
      key: "decorated",
      converter: webidl.converters["boolean"],
    },
    {
      key: "enabledButtons",
      converter: webidl.converters["unsigned long"],
    },
    {
      key: "fullscreen",
      converter: webidl.converters["boolean"],
    },
    {
      key: "position",
      converter: webidl.converters["WSIPosition"],
    },
    {
      key: "innerSize",
      converter: webidl.converters["WSISize"],
    },
    {
      key: "minInnerSize",
      converter: webidl.converters["WSISize"],
    },
    {
      key: "maxInnerSize",
      converter: webidl.converters["WSISize"],
    },
    {
      key: "level",
      converter: webidl.converters["WSIWindowLevel"],
    },
    {
      key: "maximized",
      converter: webidl.converters["boolean"],
    },
    {
      key: "resizable",
      converter: webidl.converters["boolean"],
    },
    {
      key: "resizeIncrements",
      converter: webidl.converters["WSISize"],
    },
    {
      key: "theme",
      converter: webidl.converters["WSIWindowTheme"],
    },
    {
      key: "title",
      converter: webidl.converters["DOMString"],
    },
    {
      key: "transparent",
      converter: webidl.converters["boolean"],
    },
    {
      key: "visible",
      converter: webidl.converters["boolean"],
    },
  ];
  webidl.converters["WSICreateWindowOptions"] = webidl
    .createDictionaryConverter(
      "WSICreateWindowOptions",
      dictMembersWSICreateWindowOptions,
    );
})(this);
