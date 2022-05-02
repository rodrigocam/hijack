// #[macro_use]
// extern crate lazy_static;
use std::*;
use std::alloc::{alloc, dealloc, Layout};
// use winapi::shared::hidusage::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_MOUSE};
use winapi::{
    {ctypes::c_int},
    shared::{

        ntdef::{LPCSTR, LPTSTR},
        hidusage::{HID_USAGE_PAGE_GENERIC, HID_USAGE_GENERIC_MOUSE},
        windef::{POINT, HHOOK, HWND},
        minwindef::{HINSTANCE, WPARAM, LPARAM, LRESULT, BYTE, LPVOID, UINT, TRUE, DWORD}},
        um::winuser::*,
        // um::winuser::{
        //     TranslateMessage, DispatchMessageA, MSLLHOOKSTRUCT, WH_MOUSE_LL,WH_MOUSE,
        //     HOOKPROC,SetWindowsHookExA, CallNextHookEx, PeekMessageA, MSG, PM_REMOVE,
        //     WM_MOUSEMOVE, WM_MOUSEHOVER, WM_NCMOUSEHOVER, ShowCursor, RAWINPUTDEVICE,
        //     RIDEV_INPUTSINK, RegisterRawInputDevices, RAWINPUT,PCRAWINPUTDEVICE, GetRawInputData,
        //     RID_INPUT,HRAWINPUT, RAWINPUTHEADER, WM_INPUT, RIM_TYPEMOUSE, WNDCLASSA,
        //     RegisterClassA, GetMessageA, GetRegisteredRawInputDevices,RIDEV_NOLEGACY, MB_OK,
        // },
        um::errhandlingapi::GetLastError,
        um::libloaderapi::{ GetModuleHandleW },
        um::winnt::*,
        //{LPSTR, MAKELANGID, LANG_NEUTRAL, SUBLANG_DEFAULT}
        um::minwinbase::*,
        um::winbase::*,
        //{FormatMessageA, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS},
    };

use widestring::U16CString;

const MSGFLT_ALLOW: DWORD = 1;

unsafe fn message_loop(msg: LPMSG) {
    loop {
        if GetMessageW(msg, ptr::null_mut(), 0, 0) == 0 {
            return;
        }

        TranslateMessage(msg);
        DispatchMessageW(msg);
    }
}


unsafe extern "system" fn hook(code: c_int, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    //println!("mouse moved");

    if code < 0 {
        return CallNextHookEx(std::ptr::null_mut(), code, wParam, lParam)
    }
    let data = lParam as *mut MSLLHOOKSTRUCT;
    // println!("{}, {}", (*data).pt.x, (*data).pt.y);
    //println!("{:?}", wParam);
    if wParam == WM_MOUSEMOVE as usize || wParam == WM_MOUSEHOVER as usize || wParam == WM_NCMOUSEHOVER as usize{
        // println!("{:?}", wParam);
        //println!("{}, {}", (*data).pt.x, (*data).pt.y);
        return 1;
    }
    return CallNextHookEx(std::ptr::null_mut(), -1, 0, 0);
   
    return 1;
}

unsafe extern "system" fn winproc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    println!("event");
    println!("{:?}", uMsg);
     match uMsg {
        WM_INPUT => {
            println!("YAYY");
            // let mut dwSize = std::mem::size_of::<RAWINPUT>() as u31;
            // let lpb: &mut[BYTE; std::mem::size_of::<RAWINPUT>()] = &mut[-1 as u8;std::mem::size_of::<RAWINPUT>() ];
            // GetRawInputData(lParam as HRAWINPUT, RID_INPUT, lpb.as_mut_ptr() as LPVOID, &mut dwSize, std::mem::size_of::<RAWINPUTHEADER>() as u31);
            // let raw: *const RAWINPUT = lpb.as_ptr() as *const RAWINPUT;
            // if (*raw).header.dwType == RIM_TYPEMOUSE {
            //     let xPos = (*raw).data.mouse().lLastX;
            //     let yPos = (*raw).data.mouse().lLastY;

            //     println!("{:?}, {:?}", xPos, yPos);
            // }
            return 0;
        },
        WM_QUERYENDSESSION => {
            return 0;
        },
        _ => {
            if uMsg == *WM_TASKBAR_CREATED {
                return 0;
            }
        },
    };
    DefWindowProcW(hwnd, uMsg, wParam, lParam)
}

lazy_static! {
    static ref WM_TASKBAR_CREATED: UINT = unsafe {
        RegisterWindowMessageW(U16CString::from_str("TaskbarCreated").unwrap().as_ptr())
    };

    static ref CB_SIZE_HEADER: UINT = mem::size_of::<RAWINPUTHEADER>() as UINT;
    static ref CLASS_NAME: U16CString = U16CString::from_str("W10Wheel/R_WM").unwrap();
}

fn make_raw_input_device(hwnd: HWND) -> RAWINPUTDEVICE {
    RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
    }
}

fn make_window_class(h_instance: HINSTANCE) -> WNDCLASSEXW {
    WNDCLASSEXW {
        cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hbrBackground: ptr::null_mut(),
        hCursor:  ptr::null_mut(),
        hIcon:  ptr::null_mut(),
        hIconSm:  ptr::null_mut(),
        hInstance: h_instance,
        lpfnWndProc: Some(winproc),
        lpszClassName: CLASS_NAME.as_ptr(),
        lpszMenuName: ptr::null_mut(),
        style: 0,
    }
}

pub fn set_mouse_hook() {
    unsafe {
        // let class: WNDCLASSA = WNDCLASSA{
        //     style: 0,
        //     lpfnWndProc: Some(winproc),
        //     cbClsExtra: 0,
        //     cbWndExtra: 0, 
        //     hInstance: std::ptr::null_mut(),
        //     hIcon: std::ptr::null_mut(),
        //     hCursor: std::ptr::null_mut(),
        //     hbrBackground: std::ptr::null_mut(),
        //     lpszMenuName: std::ptr::null_mut(),
        //     lpszClassName: std::ptr::null_mut(),
        // };

        // if !RegisterClassA(&class) {
        //     println!("falhouA");
        // }


        let Rid: PCRAWINPUTDEVICE = [RAWINPUTDEVICE{usUsagePage: 0x00, usUsage: 0x02, dwFlags: RIDEV_NOLEGACY, hwndTarget: std::ptr::null_mut()}].as_ptr() as *const RAWINPUTDEVICE;
        let cbsize = std::mem::size_of::<RAWINPUTDEVICE>() as u32;

        let h_instance = GetModuleHandleW(ptr::null());
        let window_class = make_window_class(h_instance);
        RegisterClassExW(&window_class);
        let hwnd = CreateWindowExW(0, CLASS_NAME.as_ptr(), ptr::null_mut(), 0, 0, 0, 0, 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
        ChangeWindowMessageFilterEx(hwnd, *WM_TASKBAR_CREATED, MSGFLT_ALLOW, ptr::null_mut());

        let rid = make_raw_input_device(hwnd);
        let mut rid_array = vec![rid];
        if RegisterRawInputDevices(rid_array.as_mut_ptr(), 1, std::mem::size_of::<RAWINPUTDEVICE>() as UINT) == 0 {
        println!("registered");
        } else {
            let dw = GetLastError();
            println!("dw {:?}", dw);
            let mut lpMsgBuf: LPWSTR = std::ptr::null_mut();
            let mut lpDisplayBuf: LPVOID = std::ptr::null_mut();

             let mut buffer: *mut u16 = std::ptr::null_mut();
            let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;

            let tchars = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER | 
                FORMAT_MESSAGE_FROM_SYSTEM |
                FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null_mut(),
                dw,
                MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as u32,
                lpBuffer,
                0,
                std::ptr::null_mut()
            );
            if tchars <= 0 && lpBuffer.is_null() {
                println!("buffer null");
            } else {
                let buffer_slice: &[u16] = core::slice::from_raw_parts(buffer, tchars as usize);
                for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
                    let ch = decode_result.unwrap_or('�');
                    println!("{}", ch);
                }
            }

            println!("{:?}", tchars );
            println!("{:?}",lstrlenA(lpBuffer as LPSTR) as usize );
            println!("{:?}", String::from_raw_parts(lpBuffer as *mut u8, lstrlenA(lpBuffer as LPSTR) as usize,lstrlenA(lpBuffer as LPSTR) as usize));
            // lpDisplayBuf = LocalAlloc(LMEM_ZEROINIT, lstrlenA(lpMsgBuf as LPSTR) as usize) as LPVOID;
            // MessageBoxA(std::ptr::null_mut(), lpDisplayBuf as LPSTR, "ERROR".as_ptr() as *const i8, MB_OK);
            // prnitln!("{:?}", FormatMessageA());
        }

        // let _hhook = SetWindowsHookExA(WH_MOUSE_LL, Some(hook), std::ptr::null_mut(), 0);
        //let _hhook = SetWindowsHookExA(WH_MOUSE_LL, Some(hook2), std::ptr::null_mut(), 0);
        //println!("installed {:?}", _hhook);


        //let mut msg: MSG = MSG{hwnd: std::ptr::null_mut(), lParam: 0, wParam: 0, message: 0, time: 0, pt: POINT{x: 1, y: 1}};
        let layout = Layout::new::<MSG>();
        let msg = alloc(layout);
        message_loop(msg as LPMSG);
        // loop {
        //     if GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) == 0{
        //         return
        //     }
        //     TranslateMessage(&msg);
            // DispatchMessageA(&msg);
            // if PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) == 0{
            //     TranslateMessage(&msg);
            //     DispatchMessageA(&msg);
            // }
        // }
    }
}

pub unsafe fn set_raw_input() {

    loop{}
}