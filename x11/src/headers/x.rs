// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::*;

pub type XID = c_ulong;
pub type Mask = c_ulong;
pub type Atom = c_ulong;
pub type VisualID = c_ulong;
pub type Time = c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type GContext = XID;
pub type KeySym = XID;
pub type KeyCode = c_uchar;
