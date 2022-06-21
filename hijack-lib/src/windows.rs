use winapi::ctypes::c_int;
use winapi::um::winuser::*;

use std::mem;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

use winapi::{
    um::{ 
        winuser::{
            RegisterRawInputDevices, DefWindowProcW, RegisterWindowMessageW, GetRawInputData,
            WNDCLASSEXW, RAWINPUTHEADER, RAWINPUTDEVICE,
            WM_INPUT, WM_QUERYENDSESSION, RID_INPUT, HRAWINPUT, RAWINPUT, RIM_TYPEMOUSE, MOUSE_MOVE_RELATIVE,
            RIDEV_INPUTSINK, RegisterClassExW, CreateWindowExW, ChangeWindowMessageFilterEx,
            GetMessageW, TranslateMessage, DispatchMessageW, MSG, LPMSG, GetCursorPos
        },
        libloaderapi::{ GetModuleHandleW },
        winnt::*,
    },
    shared::{
        windef::{ HWND, LPPOINT},
        minwindef::{ UINT, WPARAM, LPARAM, LRESULT, LPVOID, DWORD, PUINT, HINSTANCE },
        hidusage::{
            HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_MOUSE
        }
    },
};

use lazy_static::lazy_static;

use std::thread;

use widestring::U16CString;

const MSGFLT_ALLOW: DWORD = 1;
static mut MOUSE_DISABLED: bool = false;
static mut MOUSE_POS: (i32, i32) = (0, 0);

pub enum Position {
    Left,
    Right,
    Bottom,
    Top,
}

pub enum ServerStatus {
    Hijacking(Position),
    Chill,
}

static mut STATUS: ServerStatus = ServerStatus::Chill;

lazy_static! {
    static ref WM_TASKBAR_CREATED: UINT = unsafe {
        RegisterWindowMessageW(U16CString::from_str("TaskbarCreated").unwrap().as_ptr())
    };

    static ref CB_SIZE_HEADER: UINT = mem::size_of::<RAWINPUTHEADER>() as UINT;
    static ref CLASS_NAME: U16CString = U16CString::from_str("W10Wheel/R_WM").unwrap();
}

unsafe fn process_raw_input(l_param: LPARAM) -> bool {
    let mut pcb_size = 0;

    let is_mouse_move_relative = |ri: RAWINPUT| {
        ri.header.dwType == RIM_TYPEMOUSE && ri.data.mouse().usFlags == MOUSE_MOVE_RELATIVE
    };

    let get_raw_input_data = |data: LPVOID, size: PUINT| {
        GetRawInputData(l_param as HRAWINPUT, RID_INPUT, data, size, *CB_SIZE_HEADER)
    };

    if get_raw_input_data(ptr::null_mut(), &mut pcb_size) == 0 {
        let layout = Layout::from_size_align(pcb_size as usize, 1).unwrap();
        let data = alloc(layout);
        let mut res = false;

        if get_raw_input_data(data as LPVOID, &mut pcb_size) == pcb_size {
            let ri = std::ptr::read(data as *const RAWINPUT);
            if is_mouse_move_relative(ri) {
                let mouse = ri.data.mouse();
                // println!("{}, {}", mouse.lLastX, mouse.lLastY);
                MOUSE_POS.0 += mouse.lLastX;
                res = true;
            }
        }

        dealloc(data, layout);
        return false
    }

    false
}


unsafe fn message_loop(msg: LPMSG) {
    loop {
        if GetMessageW(msg, ptr::null_mut(), 0, 0) == 0 {
            return;
        }

        TranslateMessage(msg);
        DispatchMessageW(msg);
    }
}

fn make_window_class(h_instance: HINSTANCE) -> WNDCLASSEXW {

    /// Window procedure implementation to call process_raw_input.
    unsafe extern "system" fn WindowProc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
      match msg {
            WM_INPUT => {
                process_raw_input(l_param);
                return 0;
               
            },
            _ => {
                return DefWindowProcW(hwnd, msg, w_param, l_param);
            }
        };
    }
    
    WNDCLASSEXW {
        cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hbrBackground: ptr::null_mut(),
        hCursor:  ptr::null_mut(),
        hIcon:  ptr::null_mut(),
        hIconSm:  ptr::null_mut(),
        hInstance: h_instance,
        lpfnWndProc: Some(WindowProc),
        lpszClassName: CLASS_NAME.as_ptr(),
        lpszMenuName: ptr::null_mut(),
        style: 0,
    }
}

fn make_raw_input_device(hwnd: HWND) -> RAWINPUTDEVICE {
    RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
    }
}

/// Callback for Low Level mouse events. It is needed to block mouse interactions.
unsafe extern "system" fn LLMouseCallback(code: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(std::ptr::null_mut(), code, wParam, lParam)
    }
    let data = lParam as *mut MSLLHOOKSTRUCT;
    // println!("{}, {}", (*data).pt.x, (*data).pt.y);
    if (*data).pt.x < 1 {
        if !MOUSE_DISABLED {
            println!("MOVING TO LEFT SCREEN");
            STATUS = ServerStatus::Hijacking(Position::Left);
            MOUSE_DISABLED = true;
            MOUSE_POS.0 = 799;
        }
    }

    if MOUSE_POS.0 > 799 {
        if MOUSE_DISABLED {
            println!("MOVING BACK");
            STATUS = ServerStatus::Chill;
            MOUSE_DISABLED = false;
        }
    }

    if MOUSE_DISABLED {
        return 1;
    }
    //println!("{:?}", wParam);
    // if wParam == WM_MOUSEMOVE as usize || wParam == WM_MOUSEHOVER as usize || wParam == WM_NCMOUSEHOVER as usize{
        // println!("{:?}", wParam);
        //println!("{}, {}", (*data).pt.x, (*data).pt.y);
        // let arrowHandle = LoadImage(NULL, MAKEINTRESOURCE(IDC_ARROW), IMAGE_CURSOR, 0, 0, LR_SHARED);
        // let hcArrow = CopyCursor(arrowHandle);
        // SetSystemCursor(hcArrow, 32512);
        // DestroyCursor(hcArrow);
        // return 1;
    // }
    // return CallNextHookEx(std::ptr::null_mut(), -1, 0, 0);
   
    return 0;
}

fn hide_cursor() {
    unsafe {
        // let arrowHandle = LoadImage(NULL, MAKEINTRESOURCE(IDC_ARROW), IMAGE_CURSOR, 0, 0, LR_SHARED);
        // let hcArrow = CopyCursor(arrowHandle);
        // SetSystemCursor(hcArrow, 32512);
        // DestroyCursor(hcArrow);
        // let noCursor = LoadCursorFromFileW(U16CString::from_str("nocursor.cur").unwrap().as_ptr() as LPCWSTR);
        // if noCursor.is_null() {
        //     println!("{:?}", noCursor);
        //     println!("not found");
        // }
        // SetSystemCursor(noCursor, 32512);
        // SetCursor(noCursor);
    }
}

pub fn spawn_mouse_observer_thread() {
    println!("spawning observer thread");
    thread::spawn(|| {
        unsafe {
            SetCursor(std::ptr::null_mut());
            let h_instance = GetModuleHandleW(ptr::null());
            let window_class = make_window_class(h_instance);
            if RegisterClassExW(&window_class) != 0 {
                let hwnd = CreateWindowExW(0, CLASS_NAME.as_ptr(), ptr::null_mut(), 0, 0, 1000, 1000, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
                SetCursor(ptr::null_mut());
                hide_cursor();
                ChangeWindowMessageFilterEx(hwnd, *WM_TASKBAR_CREATED, MSGFLT_ALLOW, ptr::null_mut());

                let rid = make_raw_input_device(hwnd);
                let mut rid_array = vec![rid];
                RegisterRawInputDevices(rid_array.as_mut_ptr(), 1, mem::size_of::<RAWINPUTDEVICE>() as UINT);
                
                // MOUSE_DISABLED = true; 
                let mouse_hook = SetWindowsHookExA(WH_MOUSE_LL, Some(LLMouseCallback), std::ptr::null_mut(), 0);

                let layout = Layout::new::<MSG>();
                let msg = alloc(layout);
                message_loop(msg as LPMSG);
                dealloc(msg, layout);
            }
        }

    });
}
// hide cursor https://stackoverflow.com/questions/43110704/showcursorfalse-does-not-hide-cursor-on-console-application