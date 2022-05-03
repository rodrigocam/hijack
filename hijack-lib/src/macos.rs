use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::{
    event::{CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType},
    geometry::CGPoint,
};
use std::sync::mpsc;
use std::thread;

pub fn spawn_mouse_observer_thread(mouse_tx: mpsc::Sender<CGPoint>) {
    // mouse monitor thread
    thread::spawn(move || {
        println!("mouse thread spawned");
        let cur_run_loop = CFRunLoop::get_current();
        match CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            vec![CGEventType::MouseMoved /*CGEventType::LeftMouseDown*/],
            |_a, _b, event| {
                mouse_tx.send(event.location()).unwrap();
                None
            },
        ) {
            Ok(tap) => unsafe {
                let loop_source = tap.mach_port.create_runloop_source(0).unwrap();
                cur_run_loop.add_source(&loop_source, kCFRunLoopCommonModes);
                tap.enable();
                CFRunLoop::run_current();
            },
            Err(_) => panic!("event tap panicked"),
        }
    });
}
