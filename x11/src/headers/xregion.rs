// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Box {
  pub x1: c_short,
  pub x2: c_short,
  pub y1: c_short,
  pub y2: c_short,
}
pub type BOX = Box;
pub type BoxRec = Box;
pub type BoxPtr = *mut Box;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct RECTANGLE {
  pub x: c_short,
  pub y: c_short,
  pub width: c_short,
  pub height: c_short,
}
pub type RectangleRec = RECTANGLE;
pub type RectanglePtr = *mut RECTANGLE;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XRegion {
  pub size: c_long,
  pub numRects: c_long,
  pub rects: *mut BOX,
  pub extents: BOX,
}
pub type REGION = _XRegion;
