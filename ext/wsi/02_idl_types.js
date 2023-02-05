// Copyright 2023 Jo Bates. All rights reserved. MIT license.

"use strict";

((globalThis) => {
  const webidl = globalThis.__bootstrap.webidl;

  // TYPEDEF: WSIPosition
  webidl.converters.WSIPosition = webidl.createSequenceConverter(
    webidl.converters["long"],
  );

  // TYPEDEF: WSISize
  webidl.converters.WSISize = webidl.createSequenceConverter(
    webidl.converters["unsigned long"],
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
      key: "innerSize",
      converter: webidl.converters.WSISize,
    },
    {
      key: "minInnerSize",
      converter: webidl.converters.WSISize,
    },
    {
      key: "maxInnerSize",
      converter: webidl.converters.WSISize,
    },
    {
      key: "position",
      converter: webidl.converters.WSIPosition,
    },
    {
      key: "resizable",
      converter: webidl.converters["boolean"],
    },
    {
      key: "enableButtons",
      converter: webidl.converters["unsigned long"],
    },
    {
      key: "title",
      converter: webidl.converters["DOMString"],
    },
    {
      key: "fullscreen",
      converter: webidl.converters["boolean"],
    },
    {
      key: "maximized",
      converter: webidl.converters["boolean"],
    },
    {
      key: "visible",
      converter: webidl.converters["boolean"],
    },
    {
      key: "transparent",
      converter: webidl.converters["boolean"],
    },
    {
      key: "decorated",
      converter: webidl.converters["boolean"],
    },
    {
      key: "level",
      converter: webidl.converters["WSIWindowLevel"],
    },
    {
      key: "theme",
      converter: webidl.converters["WSIWindowTheme"],
    },
    {
      key: "resizeIncrements",
      converter: webidl.converters.WSISize,
    },
    {
      key: "contentProtected",
      converter: webidl.converters["boolean"],
    },
    {
      key: "active",
      converter: webidl.converters["boolean"],
    },
  ];
  webidl.converters.WSICreateWindowOptions = webidl
    .createDictionaryConverter(
      "WSICreateWindowOptions",
      dictMembersWSICreateWindowOptions,
    );
})(this);
