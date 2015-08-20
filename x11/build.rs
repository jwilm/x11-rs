// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate pkg_config;

fn main () {
  if cfg!(feature="x11") { pkg_config::find_library("x11").unwrap(); }
}
