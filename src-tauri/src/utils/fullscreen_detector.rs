pub fn is_other_window_fullscreen() -> bool {
    #[cfg(target_os = "windows")]
    return is_fullscreen_windows();

    #[cfg(target_os = "macos")]
    return is_fullscreen_macos();

    #[cfg(target_os = "linux")]
    return is_fullscreen_linux();
}

#[cfg(target_os = "windows")]
fn is_fullscreen_windows() -> bool {
    use std::ffi::c_void;
    use std::mem::zeroed;

    #[repr(C)]
    struct RECT {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct MONITORINFO {
        cb_size: u32,
        rc_monitor: RECT,
        rc_work: RECT,
        dw_flags: u32,
    }

    type HWND = *mut c_void;
    type HMONITOR = *mut c_void;

    #[link(name = "user32")]
    extern "system" {
        fn GetForegroundWindow() -> HWND;
        fn GetWindowRect(hwnd: HWND, lp_rect: *mut RECT) -> i32;
        fn MonitorFromWindow(hwnd: HWND, dw_flags: u32) -> HMONITOR;
        fn GetMonitorInfoW(h_monitor: HMONITOR, lpmi: *mut MONITORINFO) -> i32;
        fn GetDesktopWindow() -> HWND;
        fn GetShellWindow() -> HWND;
    }

    unsafe {
        let foreground = GetForegroundWindow();
        if foreground.is_null() {
            return false;
        }

        let desktop = GetDesktopWindow();
        let shell = GetShellWindow();
        if foreground == desktop || foreground == shell {
            return false;
        }

        let mut window_rect: RECT = zeroed();
        if GetWindowRect(foreground, &mut window_rect) == 0 {
            return false;
        }

        let monitor = MonitorFromWindow(foreground, 0x00000002);
        if monitor.is_null() {
            return false;
        }

        let mut monitor_info: MONITORINFO = zeroed();
        monitor_info.cb_size = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(monitor, &mut monitor_info) == 0 {
            return false;
        }

        let screen = &monitor_info.rc_monitor;
        window_rect.left <= screen.left
            && window_rect.top <= screen.top
            && window_rect.right >= screen.right
            && window_rect.bottom >= screen.bottom
    }
}

#[cfg(target_os = "macos")]
fn is_fullscreen_macos() -> bool {
    use std::ffi::c_void;
    use std::ptr::null;

    type CFTypeRef = *const c_void;

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGWindowListCopyWindowInfo(option: u32, relative_to_window: u32) -> CFTypeRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFArrayGetCount(the_array: CFTypeRef) -> isize;
        fn CFArrayGetValueAtIndex(the_array: CFTypeRef, idx: isize) -> CFTypeRef;
        fn CFDictionaryGetValue(the_dict: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
        fn CFNumberGetValue(number: CFTypeRef, the_type: isize, value_ptr: *mut c_void) -> u8;
        fn CFRelease(cf: CFTypeRef);
        fn CFStringCreateWithCString(alloc: CFTypeRef, c_str: *const i8, encoding: u32) -> CFTypeRef;
    }

    unsafe {
        let window_list = CGWindowListCopyWindowInfo(1, 0);
        if window_list.is_null() {
            return false;
        }

        let count = CFArrayGetCount(window_list);
        let layer_key = CFStringCreateWithCString(null(), b"kCGWindowLayer\0".as_ptr() as *const i8, 0x08000100);

        let mut result = false;
        for i in 0..count {
            let window_info = CFArrayGetValueAtIndex(window_list, i);
            if window_info.is_null() {
                continue;
            }

            let layer_value = CFDictionaryGetValue(window_info, layer_key);
            if !layer_value.is_null() {
                let mut layer: i32 = 0;
                if CFNumberGetValue(layer_value, 9, &mut layer as *mut i32 as *mut c_void) != 0 && layer == 0 && i == 0 {
                    result = true;
                    break;
                }
            }
        }

        CFRelease(layer_key);
        CFRelease(window_list);
        result
    }
}

#[cfg(target_os = "linux")]
fn is_fullscreen_linux() -> bool {
    use std::ffi::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};
    use std::ptr::null_mut;

    type Display = c_void;
    type Window = c_ulong;
    type Atom = c_ulong;

    #[link(name = "X11")]
    extern "C" {
        fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        fn XCloseDisplay(display: *mut Display) -> c_int;
        fn XDefaultRootWindow(display: *mut Display) -> Window;
        fn XInternAtom(display: *mut Display, atom_name: *const c_char, only_if_exists: c_int) -> Atom;
        fn XGetWindowProperty(
            display: *mut Display, w: Window, property: Atom, long_offset: c_long, long_length: c_long,
            delete: c_int, req_type: Atom, actual_type_return: *mut Atom, actual_format_return: *mut c_int,
            nitems_return: *mut c_ulong, bytes_after_return: *mut c_ulong, prop_return: *mut *mut c_uchar,
        ) -> c_int;
        fn XFree(data: *mut c_void) -> c_int;
    }

    unsafe {
        let display = XOpenDisplay(null_mut());
        if display.is_null() {
            return false;
        }

        let root = XDefaultRootWindow(display);
        let net_active_window = XInternAtom(display, b"_NET_ACTIVE_WINDOW\0".as_ptr() as *const c_char, 1);
        let net_wm_state = XInternAtom(display, b"_NET_WM_STATE\0".as_ptr() as *const c_char, 1);
        let net_wm_state_fullscreen = XInternAtom(display, b"_NET_WM_STATE_FULLSCREEN\0".as_ptr() as *const c_char, 1);

        if net_active_window == 0 || net_wm_state == 0 || net_wm_state_fullscreen == 0 {
            XCloseDisplay(display);
            return false;
        }

        let mut actual_type: Atom = 0;
        let mut actual_format: c_int = 0;
        let mut nitems: c_ulong = 0;
        let mut bytes_after: c_ulong = 0;
        let mut prop: *mut c_uchar = null_mut();

        let status = XGetWindowProperty(display, root, net_active_window, 0, 1, 0, 0, &mut actual_type, &mut actual_format, &mut nitems, &mut bytes_after, &mut prop);

        if status != 0 || nitems == 0 || prop.is_null() {
            if !prop.is_null() { XFree(prop as *mut c_void); }
            XCloseDisplay(display);
            return false;
        }

        let active_window = *(prop as *const Window);
        XFree(prop as *mut c_void);

        if active_window == 0 {
            XCloseDisplay(display);
            return false;
        }

        let status = XGetWindowProperty(display, active_window, net_wm_state, 0, 1024, 0, 0, &mut actual_type, &mut actual_format, &mut nitems, &mut bytes_after, &mut prop);

        let mut is_fullscreen = false;
        if status == 0 && nitems > 0 && !prop.is_null() {
            let atoms = std::slice::from_raw_parts(prop as *const Atom, nitems as usize);
            is_fullscreen = atoms.contains(&net_wm_state_fullscreen);
            XFree(prop as *mut c_void);
        }

        XCloseDisplay(display);
        is_fullscreen
    }
}
