// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;
use headers::xlib::*;
use headers::extensions::xkb::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbStateRec {
  pub group: c_uchar,
  pub locked_group: c_uchar,
  pub base_group: c_ushort,
  pub latched_group: c_ushort,
  pub mods: c_uchar,
  pub base_mods: c_uchar,
  pub latched_mods: c_uchar,
  pub locked_mods: c_uchar,
  pub compat_state: c_uchar,
  pub grab_mods: c_uchar,
  pub compat_grab_mods: c_uchar,
  pub lookup_mods: c_uchar,
  pub compat_lookup_mods: c_uchar,
  pub ptr_buttons: c_ushort,
}
pub type XkbStateRec = _XkbStateRec;
pub type XkbStatePtr = *mut _XkbStateRec;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbMods {
  pub mask: c_uchar,
  pub real_mods: c_uchar,
  pub vmods: c_ushort,
}
pub type XkbModsRec = _XkbMods;
pub type XkbModsPtr = *mut _XkbMods;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbKTMapEntry {
  pub active: Bool,
  pub level: c_uchar,
  pub mods: XkbModsRec,
}
pub type XkbKTMapEntryRec = _XkbKTMapEntry;
pub type XkbKTMapEntryPtr = *mut _XkbKTMapEntry;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbKeyType {
  pub mods: XkbModsRec,
  pub num_levels: c_uchar,
  pub map_count: c_uchar,
  pub map: XkbKTMapEntryPtr,
  pub preserve: XkbModsPtr,
  pub name: Atom,
  pub level_names: *mut Atom,
}
pub type XkbKeyTypeRec = _XkbKeyType;
pub type XkbKeyTypePtr = *mut _XkbKeyType;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbBehavior {
  pub type_: c_uchar,
  pub data: c_uchar,
}
pub type XkbBehavior = _XkbBehavior;

pub const XkbAnyActionDataSize: usize = 7;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbAnyAction {
  pub type_: c_uchar,
  pub data: [c_uchar; XkbAnyActionDataSize],
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbModAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub mask: c_uchar,
  pub real_mods: c_uchar,
  pub vmods1: c_uchar,
  pub vmods2: c_uchar,
}
pub type XkbModAction = _XkbModAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbGroupAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub group_XXX: c_char,
}
pub type XkbGroupAction = _XkbGroupAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbISOAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub mask: c_uchar,
  pub real_mods: c_uchar,
  pub group_XXX: c_char,
  pub affect: c_uchar,
  pub vmods1: c_uchar,
  pub vmods2: c_uchar,
}
pub type XkbISOAction = _XkbISOAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbPtrAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub high_XXX: c_uchar,
  pub low_XXX: c_uchar,
  pub high_YYY: c_uchar,
  pub low_YYY: c_uchar,
}
pub type XkbPtrAction = _XkbPtrAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbPtrBtnAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub count: c_uchar,
  pub button: c_uchar,
}
pub type XkbPtrBtnAction = _XkbPtrBtnAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbPtrDfltAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub affect: c_uchar,
  pub valueXXX: c_char,
}
pub type XkbPtrDfltAction = _XkbPtrDfltAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbSwitchScreenAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub screen_XXX: c_char,
}
pub type XkbSwitchScreenAction = _XkbSwitchScreenAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbCtrlsAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub ctrls3: c_uchar,
  pub ctrls2: c_uchar,
  pub ctrls1: c_uchar,
  pub ctrls0: c_uchar,
}
pub type XkbCtrlsAction = _XkbCtrlsAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbMessageAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub message: [c_uchar; 6],
}
pub type XkbMessageAction = _XkbMessageAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbRedirectKeyAction {
  pub type_: c_uchar,
  pub new_key: c_uchar,
  pub mods_mask: c_uchar,
  pub mods: c_uchar,
  pub vmods_mask0: c_uchar,
  pub vmods_mask1: c_uchar,
  pub vmods0: c_uchar,
  pub vmods1: c_uchar,
}
pub type XkbRedirectKeyAction = _XkbRedirectKeyAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbDeviceBtnAction {
  pub type_: c_uchar,
  pub flags: c_uchar,
  pub count: c_uchar,
  pub button: c_uchar,
  pub device: c_uchar,
}
pub type XkbDeviceBtnAction = _XkbDeviceBtnAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbDeviceValuatorAction {
  pub type_: c_uchar,
  pub device: c_uchar,
  pub v1_what: c_uchar,
  pub v1_ndx: c_uchar,
  pub v1_value: c_uchar,
  pub v2_what: c_uchar,
  pub v2_ndx: c_uchar,
  pub v2_value: c_uchar,
}
pub type XkbDeviceValuatorAction = _XkbDeviceValuatorAction;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbAction { // union of Xkb*Action structs
  _data: [c_char; 8],
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbControls {
  pub mk_dflt_btn: c_uchar,
  pub num_groups: c_uchar,
  pub groups_wrap: c_uchar,
  pub internal: XkbModsRec,
  pub ignore_lock: XkbModsRec,
  pub enabled_ctrls: c_uint,
  pub repeat_delay: c_ushort,
  pub repeat_interval: c_ushort,
  pub slow_keys_delay: c_ushort,
  pub debounce_delay: c_ushort,
  pub mk_delay: c_ushort,
  pub mk_interval: c_ushort,
  pub mk_time_to_max: c_ushort,
  pub mk_max_speed: c_ushort,
  pub mk_curve: c_short,
  pub ax_options: c_ushort,
  pub ax_timeout: c_ushort,
  pub axt_opts_mask: c_ushort,
  pub axt_opts_values: c_ushort,
  pub axt_ctrls_mask: c_uint,
  pub axt_ctrls_values: c_uint,
  pub per_key_repeat: [c_uchar; XkbPerKeyBitArraySize],
}
pub type XkbControlsRec = _XkbControls;
pub type XkbControlsPtr = *mut _XkbControls;

//_XkbServerMapRec
//_XkbSymMapRec
//_XkbClientMapRec
//_XkbSymInterpretRec
//_XkbCompatMapRec

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbIndicatorMapRec {
  pub flags: c_uchar,
  pub which_groups: c_uchar,
  pub groups: c_uchar,
  pub which_mods: c_uchar,
  pub mods: XkbModsRec,
  pub ctrls: c_uint,
}
pub type XkbIndicatorMapRec = _XkbIndicatorMapRec;
pub type XkbIndicatorMapPtr = *mut _XkbIndicatorMapRec;

//_XkbIndicatorRec
//_XkbKeyNameRec
//_XkbKeyAliasRec
//_XkbNamesRec

pub enum _XkbGeometry {}
pub type XkbGeometry = _XkbGeometry;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbDesc {
  pub dpy: *mut Display,
  pub flags: c_ushort,
  pub device_spec: c_ushort,
  pub min_key_code: KeyCode,
  pub max_key_code: KeyCode,
  pub ctrls: XkbControlsPtr,
  pub server: XkbServerMapPtr,
  pub map: XkbClientMapPtr,
  pub indicators: XkbIndicatorPtr,
  pub names: XkbNamesPtr,
  pub compat: XkbCompatMapPtr,
  pub geom: XkbGeometryPtr,
}
pub type XkbDescRec = _XkbDesc;
pub type XkbDescPtr = *mut _XkbDesc;

//_XkbMapChanges
//_XkbControlsChanges
//_XkbIndicatorChanges
//_XkbNameChanges
//_XkbCompatChanges

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbChanges {
  pub device_spec: c_ushort,
  pub state_changes: c_ushort,
  pub map: XkbMapChangesRec,
  pub ctrls: XkbControlsChangesRec,
  pub indicators: XkbIndicatorChangesRec,
  pub names: XkbNameChangesRec,
  pub compat: XkbCompatChangesRec,
}
pub type XkbChangesRec = _XkbChanges;
pub type XkbChangesPtr = *mut _XkbChanges;

//_XkbComponentNames
//_XkbComponentName
//_XkbComponentList
//_XkbDeviceLedInfo

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XkbDeviceInfo {
  pub name: *mut c_char,
  pub type_: Atom,
  pub device_spec: c_ushort,
  pub has_own_state: Bool,
  pub supported: c_ushort,
  pub unsupported: c_ushort,
  pub num_btns: c_ushort,
  pub btn_acts: *mut XkbAction,
  pub sz_leds: c_ushort,
  pub num_leds: c_ushort,
  pub dflt_kbd_fb: c_ushort,
  pub dflt_led_fb: c_ushort,
  pub leds: XkbDeviceLedInfoPtr,
}
pub type XkbDeviceInfoRec = _XkbDeviceInfo;
pub type XkbDeviceInfoPtr = *mut _XkbDeviceInfo;

//_XkbDeviceLedChanges
//_XkbDeviceChanges
