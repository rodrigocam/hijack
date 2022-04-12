use std::ffi::CString;

use core_foundation::{
    base::{kCFAllocatorDefault, ToVoid},
    boolean::kCFBooleanTrue,
    runloop::{kCFRunLoopCommonModes, CFRunLoop},
    string::{kCFStringEncodingUTF8, CFStringCreateWithCString, CFStringRef},
};
use core_graphics::{
    display::{CGSSetConnectionProperty, _CGSDefaultConnection},
    event::{
        CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy,
        CGEventType,
    },
};

pub struct MouseObserver {
    #[cfg(target_os = "macos")]
    run_loop: CFRunLoop,
}

impl MouseObserver {
    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        unsafe {
            let cstr = CString::new("SetsCursorInBackground").unwrap();
            let cstr_ptr = cstr.as_ptr() as *const i8;

            let property_string: CFStringRef =
                CFStringCreateWithCString(kCFAllocatorDefault, cstr_ptr, kCFStringEncodingUTF8);

            CGSSetConnectionProperty(
                _CGSDefaultConnection(),
                _CGSDefaultConnection(),
                property_string,
                kCFBooleanTrue.to_void(),
            );
        }
        Self {
            run_loop: CFRunLoop::get_current(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn run(self) {
        match CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            vec![CGEventType::MouseMoved, CGEventType::LeftMouseDown],
            |_a, _b, event| {
                if event.location().x == 0.0 {
                    println!("{:?}", event.location());
                    // CGDisplay::hide_cursor(&CGDisplay::main()).unwrap();
                }
                event.set_type(CGEventType::Null);
                None
            },
        ) {
            Ok(tap) => unsafe {
                let loop_source = tap
                    .mach_port
                    .create_runloop_source(0)
                    .expect("Error creatig loop source");
                self.run_loop
                    .add_source(&loop_source, kCFRunLoopCommonModes);
                tap.enable();
                CFRunLoop::run_current();
            },
            Err(_) => println!("err"),
        }
    }
}
