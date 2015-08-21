// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;
use headers::x::*;

pub type Bool = c_int;
pub type Status = c_int;
pub const True: Bool = 1;
pub const False: Bool = 0;

pub enum _XDisplay {}
pub type Display = _XDisplay;

pub type XPointer = *mut c_char;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XRectangle {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}

pub enum _XGC {}
pub type GC = *mut _XGC;

pub enum _XIC {}
pub type XIC = *mut _XIC;

pub enum _XIM {}
pub type XIM = *mut _XIM;

pub enum _XOC {}
pub type XOC = *mut _XOC;
pub type XFontSet = *mut _XOC;

pub enum _XOM {}
pub type XOM = *mut _XOM;

pub type XICCEncodingStyle = c_int; // enum
pub const XStringStyle: XICCEncodingStyle = 0;
pub const XCompoundTextStyle: XICCEncodingStyle = 1;
pub const XTextStyle: XICCEncodingStyle = 2;
pub const XStdICCTextStyle: XICCEncodingStyle = 3;
pub const XUTF8StringStyle: XICCEncodingStyle = 4;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XmbTextItem {
  pub chars: *mut c_char,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XwcTextItem {
  pub chars: *mut wchar_t,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XGenericEventCookie {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub cookie: c_uint,
  pub data: *mut c_void,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XExtCodes {
  pub extension: c_int,
  pub major_opcode: c_int,
  pub first_event: c_int,
  pub first_error: c_int,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XEvent { // union of X*Event structs
  _data: [c_long; 24],
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XErrorEvent {
  pub type_: c_int,
  pub display: *mut Display,
  pub resourceid: XID,
  pub serial: c_ulong,
  pub error_code: c_uchar,
  pub request_code: c_uchar,
  pub minor_code: c_uchar,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XKeyEvent {
  pub type_: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: Bool,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XExtData {
  pub number: c_int,
  pub next: *mut _XExtData,
  pub free_private: Option<extern fn (*mut _XExtData) -> c_int>,
  pub private_data: XPointer,
}
pub type XExtData = _XExtData;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XTextItem {
  pub chars: *mut c_char,
  pub nchars: c_int,
  pub delta: c_int,
  pub font: Font,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XChar2b {
  pub byte1: c_uchar,
  pub byte2: c_uchar,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XTextItem16 {
  pub chars: *mut XChar2b,
  pub nchars: c_int,
  pub delta: c_int,
  pub font: Font,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Screen {
  pub ext_data: *mut XExtData,
  pub display: *mut Display,
  pub root: Window,
  pub width: c_int,
  pub height: c_int,
  pub mwidth: c_int,
  pub mheight: c_int,
  pub ndepths: c_int,
  pub depths: *mut Depth,
  pub root_depth: c_int,
  pub root_visual: *mut Visual,
  pub default_gc: GC,
  pub cmap: Colormap,
  pub white_pixel: c_ulong,
  pub black_pixel: c_ulong,
  pub max_maps: c_int,
  pub min_maps: c_int,
  pub backing_store: c_int,
  pub save_unders: Bool,
  pub root_input_mask: c_long,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XColor {
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Visual {
  pub ext_data: *mut XExtData,
  pub visualid: VisualID,
  pub class: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub bits_per_rgb: c_int,
  pub map_entries: c_int,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Depth {
  pub depth: c_int,
  pub nvisuals: c_int,
  pub visuals: *mut Visual,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XHostAddress {
  pub family: c_int,
  pub length: c_int,
  pub address: *mut c_char,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XGCValues {
  pub function: c_int,
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub line_width: c_int,
  pub line_style: c_int,
  pub cap_style: c_int,
  pub join_style: c_int,
  pub fill_style: c_int,
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,
  pub subwindow_mode: c_int,
  pub graphics_exposures: Bool,
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,
  pub dash_offset: c_int,
  pub dashes: c_char,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XKeyboardControl {
  pub key_click_percent: c_int,
  pub bell_percent: c_int,
  pub bell_pitch: c_int,
  pub bell_duration: c_int,
  pub led: c_int,
  pub led_mode: c_int,
  pub key: c_int,
  pub auto_repeat_mode: c_int,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XSetWindowAttributes {
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub colormap: Colormap,
  pub cursor: Cursor,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *mut Visual,
  pub root: Window,
  pub class: c_int,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub colormap: Colormap,
  pub map_installed: Bool,
  pub map_state: c_int,
  pub all_event_masks: c_long,
  pub your_event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub screen: *mut Screen,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct XWindowChanges {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub sibling: Window,
  pub stack_mode: c_int,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct _XImage {
  pub width: c_int,
  pub height: c_int,
  pub xoffset: c_int,
  pub format: c_int,
  pub data: *mut c_char,
  pub byte_order: c_int,
  pub bitmap_unit: c_int,
  pub bitmap_bit_order: c_int,
  pub bitmap_pad: c_int,
  pub depth: c_int,
  pub bytes_per_line: c_int,
  pub bits_per_pixel: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub obdata: XPointer,
  pub create_image: Option<extern fn (*mut Display, *mut Visual, c_uint, c_int, c_int, *mut c_char, c_uint, c_uint, c_int, c_int) -> *mut _XImage>,
  pub destroy_image: Option<extern fn (*mut _XImage) -> c_int>,
  pub get_pixel: Option<extern fn (*mut _XImage, c_int, c_int) -> c_ulong>,
  pub put_pixel: Option<extern fn (*mut _XImage, c_int, c_int, c_ulong) -> c_int>,
  pub sub_image: Option<extern fn (*mut _XImage, c_int, c_int, c_uint, c_uint) -> *mut _XImage>,
  pub add_pixel: Option<extern fn (*mut _XImage, c_long) -> c_int>,
}
pub type XImage = _XImage;
