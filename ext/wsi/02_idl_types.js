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
      key: "alwaysOnTop",
      converter: webidl.converters["boolean"],
    },
  ];
  webidl.converters.WSICreateWindowOptions = webidl
    .createDictionaryConverter(
      "WSICreateWindowOptions",
      dictMembersWSICreateWindowOptions,
    );
})(this);
