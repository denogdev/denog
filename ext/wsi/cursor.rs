use serde::Deserialize;
use winit::window::{CursorGrabMode, CursorIcon};

#[derive(Deserialize)]
pub struct WsiCursorGrabMode(
  #[serde(with = "WsiCursorGrabModeDef")] pub CursorGrabMode,
);

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", remote = "CursorGrabMode")]
enum WsiCursorGrabModeDef {
  None,
  Confined,
  Locked,
}

#[derive(Deserialize)]
pub struct WsiCursorIcon(#[serde(with = "WsiCursorIconDef")] pub CursorIcon);

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", remote = "CursorIcon")]
enum WsiCursorIconDef {
  Default,
  Crosshair,
  Hand,
  Arrow,
  Move,
  Text,
  Wait,
  Help,
  Progress,
  NotAllowed,
  ContextMenu,
  Cell,
  VerticalText,
  Alias,
  Copy,
  NoDrop,
  Grab,
  Grabbing,
  AllScroll,
  ZoomIn,
  ZoomOut,
  EResize,
  NResize,
  NeResize,
  NwResize,
  SResize,
  SeResize,
  SwResize,
  WResize,
  EwResize,
  NsResize,
  NeswResize,
  NwseResize,
  ColResize,
  RowResize,
}
