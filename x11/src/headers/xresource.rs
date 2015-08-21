// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;
use headers::xlib::*;

pub type XrmBinding = c_int; // enum
pub type XrmBindingList = *mut XrmBinding;
pub const XrmBindTightly: XrmBinding = 0;
pub const XrmBindLoosely: XrmBinding = 1;

pub enum _XrmHashBucketRec {}
pub type XrmHashBucket = *mut _XrmHashBucketRec;
pub type XrmDatabase = *mut _XrmHashBucketRec;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XrmValue {
  pub size: c_uint,
  pub addr: XPointer,
}
pub type XrmValuePtr = *mut XrmValue;

pub type XrmOptionKind = c_int; // enum
pub const XrmoptionNoArg: XrmOptionKind = 0;
pub const XrmoptionIsArg: XrmOptionKind = 1;
pub const XrmoptionStickyArg: XrmOptionKind = 2;
pub const XrmoptionSepArg: XrmOptionKind = 3;
pub const XrmoptionResArg: XrmOptionKind = 4;
pub const XrmoptionSkipArg: XrmOptionKind = 5;
pub const XrmoptionSkipLine: XrmOptionKind = 6;
pub const XrmoptionSkipNArgs: XrmOptionKind = 7;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XrmOptionDescRec {
  pub option: *mut c_char,
  pub specifier: *mut c_char,
  pub argKind: XrmOptionKind,
  pub value: XPointer,
}
pub type XrmOptionDescList = *mut XrmOptionDescRec;
