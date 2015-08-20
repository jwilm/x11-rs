// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#[repr(C)]
enum TestEnum { A, B }

#[test]
fn enum_size_test () {
  use std::mem;
  use libc::c_int;
  assert!(mem::size_of::<TestEnum>() == mem::size_of::<c_int>());
}
