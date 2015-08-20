// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XTextProperty {
  pub value: *mut c_uchar,
  pub encoding: Atom,
  pub format: c_int,
  pub nitems: c_ulong,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XClassHint {
  pub res_name: *mut c_char,
  pub res_class: *mut c_char,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XWMHints {
  pub flags: c_long,
  pub input: Bool,
  pub initial_state: c_int,
  pub icon_pixmap: Pixmap,
  pub icon_window: Window,
  pub icon_x: c_int,
  pub icon_y: c_int,
  pub icon_mask: Pixmap,
  pub window_group: XID,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XSizeHints {
  pub flags: c_long,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub min_width: c_int,
  pub min_height: c_int,
  pub max_width: c_int,
  pub max_height: c_int,
  pub width_inc: c_int,
  pub height_inc: c_int,
  pub min_aspect_x: c_int,
  pub min_aspect_y: c_int,
  pub max_aspect_x: c_int,
  pub max_aspect_y: c_int,
  pub base_width: c_int,
  pub base_height: c_int,
  pub win_gravity: c_int,
}
