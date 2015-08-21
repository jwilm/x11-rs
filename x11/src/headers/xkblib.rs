// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;
use headers::xlib::*;

//_XkbAnyEvent
//_XkbNewKeyboardNotify

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbMapNotifyEvent {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub time: Time,
  pub xkb_type: c_int,
  pub device: c_int,
  pub changed: c_uint,
  pub flags: c_uint,
  pub first_type: c_int,
  pub num_types: c_int,
  pub min_key_code: KeyCode,
  pub max_key_code: KeyCode,
  pub first_key_sym: KeyCode,
  pub first_key_act: KeyCode,
  pub first_key_behavior: KeyCode,
  pub first_key_explicit: KeyCode,
  pub first_modmap_key: KeyCode,
  pub first_vmodmap_key: KeyCode,
  pub num_key_syms: c_int,
  pub num_key_acts: c_int,
  pub num_key_behaviors: c_int,
  pub num_key_explicit: c_int,
  pub num_modmap_keys: c_int,
  pub num_vmodmap_keys: c_int,
  pub vmods: c_uint,
}
pub type XkbMapNotifyEvent = _XkbMapNotifyEvent;

//_XkbStateNotifyEvent
//_XkbControlsNotify
//_XkbIndicatorNotify

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbNamesNotify {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub time: Time,
  pub xkb_type: c_int,
  pub device: c_int,
  pub changed: c_uint,
  pub first_type: c_int,
  pub num_types: c_int,
  pub first_lvl: c_int,
  pub num_lvls: c_int,
  pub num_aliases: c_int,
  pub num_radio_groups: c_int,
  pub changed_vmods: c_uint,
  pub changed_groups: c_uint,
  pub changed_indicators: c_uint,
  pub first_key: c_int,
  pub num_keys: c_int,
}
pub type XkbNamesNotifyEvent = _XkbNamesNotify;

//_XkbCompatMapNotify
//_XkbBellNotify
//_XkbActionMessage
//_XkbAccessXNotify

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbExtensionDeviceNotify {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub time: Time,
  pub xkb_type: c_int,
  pub device: c_int,
  pub reason: c_uint,
  pub supported: c_uint,
  pub unsupported: c_uint,
  pub first_btn: c_int,
  pub num_btns: c_int,
  pub leds_defined: c_uint,
  pub led_state: c_uint,
  pub led_class: c_int,
  pub led_id: c_int,
}
pub type XkbExtensionDeviceNotifyEvent = _XkbExtensionDeviceNotify;

//_XkbEvent
//_XkbKbdDpyState
