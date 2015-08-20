// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate libc;

#[allow(raw_pointer_derive)]
mod headers;
mod libs;
#[cfg(test)]
mod tests;

pub use headers::*;
pub use libs::*;
