// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;

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

//

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
