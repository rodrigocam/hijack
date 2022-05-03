use winapi::ctypes::c_int;
use winapi::um::winuser::*;

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HINSTANCE, LPARAM, LPVOID, LRESULT, PUINT, UINT, WPARAM},
        windef::HWND,
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winnt::*,
        winuser::{
            ChangeWindowMessageFilterEx, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GetMessageW, GetRawInputData, RegisterClassExW, RegisterRawInputDevices,
            RegisterWindowMessageW, TranslateMessage, HRAWINPUT, LPMSG, MOUSE_MOVE_RELATIVE, MSG,
            RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT, RIM_TYPEMOUSE,
            WM_INPUT, WM_QUERYENDSESSION, WNDCLASSEXW,
        },
    },
};

use widestring::U16CString;

pub fn spawn_mouse_observer_thread() {
    unsafe {
        //SetCursor(std::ptr::null_mut());
        let h_instance = GetModuleHandleW(ptr::null());
        let window_class = make_window_class(h_instance);
        if RegisterClassExW(&window_class) != 0 {
            let hwnd = CreateWindowExW(
                0,
                CLASS_NAME.as_ptr(),
                ptr::null_mut(),
                0,
                0,
                1000,
                1000,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            ChangeWindowMessageFilterEx(hwnd, *WM_TASKBAR_CREATED, MSGFLT_ALLOW, ptr::null_mut());

            let rid = make_raw_input_device(hwnd);
            let mut rid_array = vec![rid];
            RegisterRawInputDevices(
                rid_array.as_mut_ptr(),
                1,
                mem::size_of::<RAWINPUTDEVICE>() as UINT,
            );

            let _hhook = SetWindowsHookExA(WH_MOUSE_LL, Some(hook), std::ptr::null_mut(), 0);

            let layout = Layout::new::<MSG>();
            let msg = alloc(layout);
            message_loop(msg as LPMSG);
            dealloc(msg, layout);
        }
    }
}
