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

#[test]
fn xkb_action_size_test () {
  use std::cmp;
  use std::mem;
  use headers::extensions::xkbstr::*;

  let sizes = [
    mem::size_of::<XkbAnyAction>(),
    mem::size_of::<XkbModAction>(),
    mem::size_of::<XkbGroupAction>(),
    mem::size_of::<XkbISOAction>(),
    mem::size_of::<XkbPtrAction>(),
    mem::size_of::<XkbPtrBtnAction>(),
    mem::size_of::<XkbPtrDfltAction>(),
    mem::size_of::<XkbSwitchScreenAction>(),
    mem::size_of::<XkbCtrlsAction>(),
    mem::size_of::<XkbMessageAction>(),
    mem::size_of::<XkbRedirectKeyAction>(),
    mem::size_of::<XkbDeviceBtnAction>(),
    mem::size_of::<XkbDeviceValuatorAction>(),
  ];
  let max_size = sizes.iter().cloned().fold(0, |a, b| cmp::max(a, b));
  assert!(mem::size_of::<XkbAction>() == max_size);
}
