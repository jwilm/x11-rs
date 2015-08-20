// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate libc;
extern crate x11;

mod libs;

pub use libs::*;
