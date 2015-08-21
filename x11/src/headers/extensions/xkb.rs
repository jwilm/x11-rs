// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

pub const XkbMaxLegalKeyCode: usize = 255;
pub const XkbPerKeyBitArraySize: usize = ((XkbMaxLegalKeyCode + 1) / 8);
