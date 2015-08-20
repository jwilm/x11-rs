// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;

pub enum _XDisplay {}
pub type Display = _XDisplay;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XRectangle {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}

pub enum _XOC {}
pub type XOC = *mut _XOC;
pub type XFontSet = *mut _XOC;

pub enum _XIC {}
pub type XIC = *mut _XIC;

pub type XICCEncodingStyle = c_int; // enum
pub const XStringStyle: XICCEncodingStyle = 0;
pub const XCompoundTextStyle: XICCEncodingStyle = 1;
pub const XTextStyle: XICCEncodingStyle = 2;
pub const XStdICCTextStyle: XICCEncodingStyle = 3;
pub const XUTF8StringStyle: XICCEncodingStyle = 4;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XmbTextItem {
  pub chars: *mut c_char,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XwcTextItem {
  pub chars: *mut wchar_t,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

pub enum _XGC {}
pub type GC = *mut _XGC;


//
// Events
//


#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XKeyEvent {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: Bool,
}
