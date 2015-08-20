// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#[cfg(feature="x11")]
mod x11;
#[cfg(feature="x11")]
pub use self::x11::*;
