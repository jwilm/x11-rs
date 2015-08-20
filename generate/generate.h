#pragma once

#include <iostream>
#include <string>

#define XUTIL_DEFINE_FUNCTIONS

#include <X11/X.h>
#include <X11/Xcms.h>
#include <X11/XKBlib.h>
#include <X11/Xlib.h>
#include <X11/Xlibint.h>
#include <X11/Xresource.h>
#include <X11/Xutil.h>

#include <X11/extensions/XKBgeom.h>


//
// Type
//


template<typename T>
struct Type;

template<>
struct Type<void> {
  static std::string name () { return "c_void"; }
};

template<>
struct Type<char> {
  static std::string name () { return "c_char"; }
};

template<>
struct Type<signed char> {
  static std::string name () { return "c_schar"; }
};

template<>
struct Type<unsigned char> {
  static std::string name () { return "c_uchar"; }
};

template<>
struct Type<signed short> {
  static std::string name () { return "c_short"; }
};

template<>
struct Type<unsigned short> {
  static std::string name () { return "c_ushort"; }
};

template<>
struct Type<signed int> {
  static std::string name () { return "c_int"; }
};

template<>
struct Type<unsigned int> {
  static std::string name () { return "c_uint"; }
};

template<>
struct Type<signed long> {
  static std::string name () { return "c_long"; }
};

template<>
struct Type<unsigned long> {
  static std::string name () { return "c_ulong"; }
};

template<>
struct Type<signed long long> {
  static std::string name () { return "c_longlong"; }
};

template<>
struct Type<unsigned long long> {
  static std::string name () { return "c_ulonglong"; }
};

template<>
struct Type<float> {
  static std::string name () { return "c_float"; }
};

template<>
struct Type<double> {
  static std::string name () { return "c_double"; }
};

template<>
struct Type<wchar_t> {
  static std::string name () { return "wchar_t"; }
};

template<typename T>
struct Type<T*> {
  static std::string name () { return std::string("*mut ") + Type<T>::name(); }
};

template<typename T>
struct Type<const T*> {
  static std::string name () { return std::string("*const ") + Type<T>::name(); }
};

#define DECLARE_TYPE(T_) \
  template<> \
  struct Type<T_> { \
    static std::string name () { return #T_; } \
  };

DECLARE_TYPE(_XcmsCCC)
DECLARE_TYPE(_XcmsColorSpace)
DECLARE_TYPE(_XcmsFunctionSet)
DECLARE_TYPE(_XComposeStatus)
DECLARE_TYPE(_XExtData)
DECLARE_TYPE(_XGC)
DECLARE_TYPE(_XIC)
DECLARE_TYPE(_XIM)
DECLARE_TYPE(_XImage)
DECLARE_TYPE(_XkbAction)
DECLARE_TYPE(_XkbBounds)
DECLARE_TYPE(_XkbChanges)
DECLARE_TYPE(_XkbColor)
DECLARE_TYPE(_XkbComponentList)
DECLARE_TYPE(_XkbComponentNames)
DECLARE_TYPE(_XkbControlsChanges)
DECLARE_TYPE(_XkbControlsNotify)
DECLARE_TYPE(_XkbDesc)
DECLARE_TYPE(_XkbDeviceChanges)
DECLARE_TYPE(_XkbDeviceInfo)
DECLARE_TYPE(_XkbDeviceLedInfo)
DECLARE_TYPE(_XkbDoodad)
DECLARE_TYPE(_XkbExtensionDeviceNotify)
DECLARE_TYPE(_XkbGeometry)
DECLARE_TYPE(_XkbGeometrySizes)
DECLARE_TYPE(_XkbIndicatorMapRec)
DECLARE_TYPE(_XkbKey)
DECLARE_TYPE(_XkbKeyAliasRec)
DECLARE_TYPE(_XkbKeyType)
DECLARE_TYPE(_XkbMapChanges)
DECLARE_TYPE(_XkbMapNotifyEvent)
DECLARE_TYPE(_XkbNameChanges)
DECLARE_TYPE(_XkbNamesNotify)
DECLARE_TYPE(_XkbOutline)
DECLARE_TYPE(_XkbOverlay)
DECLARE_TYPE(_XkbOverlayKey)
DECLARE_TYPE(_XkbOverlayRow)
DECLARE_TYPE(_XkbProperty)
DECLARE_TYPE(_XkbRow)
DECLARE_TYPE(_XkbSection)
DECLARE_TYPE(_XkbShape)
DECLARE_TYPE(_XkbStateRec)
DECLARE_TYPE(_XOC)
DECLARE_TYPE(_XOM)
DECLARE_TYPE(_XRegion)
DECLARE_TYPE(_XrmHashBucketRec)
DECLARE_TYPE(Display)
DECLARE_TYPE(Screen)
DECLARE_TYPE(Visual)
DECLARE_TYPE(XArc)
DECLARE_TYPE(XChar2b)
DECLARE_TYPE(XCharStruct)
DECLARE_TYPE(XClassHint)
DECLARE_TYPE(XcmsColor)
DECLARE_TYPE(XColor)
DECLARE_TYPE(XEDataObject)
DECLARE_TYPE(xError)
DECLARE_TYPE(XErrorEvent)
DECLARE_TYPE(XEvent)
DECLARE_TYPE(xEvent)
DECLARE_TYPE(XExtCodes)
DECLARE_TYPE(XFontSetExtents)
DECLARE_TYPE(XFontStruct)
DECLARE_TYPE(XGCValues)
DECLARE_TYPE(XGenericEventCookie)
DECLARE_TYPE(XHostAddress)
DECLARE_TYPE(XICCEncodingStyle)
DECLARE_TYPE(XIconSize)
DECLARE_TYPE(XKeyboardControl)
DECLARE_TYPE(XKeyboardState)
DECLARE_TYPE(XKeyEvent)
DECLARE_TYPE(XMappingEvent)
DECLARE_TYPE(XmbTextItem)
DECLARE_TYPE(XModifierKeymap)
DECLARE_TYPE(XPixmapFormatValues)
DECLARE_TYPE(XPoint)
DECLARE_TYPE(XRectangle)
DECLARE_TYPE(XrmBinding)
DECLARE_TYPE(XrmOptionDescRec)
DECLARE_TYPE(XrmValue)
DECLARE_TYPE(XSegment)
DECLARE_TYPE(XSetWindowAttributes)
DECLARE_TYPE(XSizeHints)
DECLARE_TYPE(XStandardColormap)
DECLARE_TYPE(XTextItem)
DECLARE_TYPE(XTextItem16)
DECLARE_TYPE(XTextProperty)
DECLARE_TYPE(XTimeCoord)
DECLARE_TYPE(XVisualInfo)
DECLARE_TYPE(XwcTextItem)
DECLARE_TYPE(XWindowAttributes)
DECLARE_TYPE(XWindowChanges)
DECLARE_TYPE(XWMHints)


//
// ParamList
//


template<typename...T>
struct ParamList;

template<>
struct ParamList<> {
  static constexpr int COUNT = 0;
  static std::string params () { return ""; }
  static std::string types () { return ""; }
};

template<typename T>
struct ParamList<T> {
  static constexpr int COUNT = 1;
  static std::string params () { return std::string("_1: ") + Type<T>::name(); }
  static std::string types () { return Type<T>::name(); }
};

template<typename Head, typename...Tail>
struct ParamList<Head, Tail...> {
  static constexpr int COUNT = ParamList<Tail...>::COUNT + 1;
  static std::string params () { return std::string("_") + std::to_string(COUNT) + ": " + Type<Head>::name() + ", " + ParamList<Tail...>::params(); }
  static std::string types () { return Type<Head>::name() + ", " + ParamList<Tail...>::types(); }
};


//
// Type specialization for function pointers
//


template<typename...Params>
struct Type<void(*)(Params...)> {
  static std::string name () { return std::string("Option<extern fn (") + ParamList<Params...>::types() + ")>"; }
};

template<typename...Params>
struct Type<void(*)(Params..., ...)> {
  static std::string name () { return std::string("Option<extern fn (") + ParamList<Params...>::types() + ", ...)>"; }
};

template<typename R, typename...Params>
struct Type<R(*)(Params...)> {
  static std::string name () { return std::string("Option<extern fn (") + ParamList<Params...>::types() + ") -> " + Type<R>::name() + ">"; }
};

template<typename R, typename...Params>
struct Type<R(*)(Params..., ...)> {
  static std::string name () { return std::string("Option<extern fn (") + ParamList<Params...>::types() + ", ...) -> " + Type<R>::name() + ">"; }
};


//
// Function
//


template<typename T>
struct Function;

template<typename...Params>
struct Function<void(*)(Params...)> {
  static void declare_static (const char *name) { std::cout << "pub fn " << name << " (" << ParamList<Params...>::params() << ");" << std::endl; }
};

template<typename...Params>
struct Function<void(*)(Params..., ...)> {
  static void declare_static (const char *name) { std::cout << "pub fn " << name << " (" << ParamList<Params...>::params() << ", ...);" << std::endl; }
};

template<typename R, typename...Params>
struct Function<R(*)(Params...)> {
  static void declare_static (const char *name) { std::cout << "pub fn " << name << " (" << ParamList<Params...>::params() << ") -> " << Type<R>::name() << ";" << std::endl; }
};

template<typename R, typename...Params>
struct Function<R(*)(Params..., ...)> {
  static void declare_static (const char *name) { std::cout << "pub fn " << name << " (" << ParamList<Params...>::params() << ", ...) -> " << Type<R>::name() << ";" << std::endl; }
};


//
// global functions
//


template<typename T>
void declare_static (T func, const char *name) {
  Function<T>::declare_static(name);
}
