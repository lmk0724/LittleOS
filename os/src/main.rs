#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
mod lang_items;
mod sbi;
mod console;
mod batch;
mod sync;
mod trap;
mod syscall;
mod loader;
mod config;
mod task;
mod timer;
mod mm;
extern crate alloc;
#[macro_use]
extern crate bitflags;
use core::arch::global_asm;

use crate::{loader::load_apps, task::run_first_task, mm::{init_heap, heap_test, init_frame_allocator, frame_allocator_test}};

global_asm!(include_str!("entry.asm"));
// fn main() {
//     // println!("Hello, world!");
// }
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("2221");
    mm::remap_test();
    println!("2222");

    trap::init();

    trap::enable_timer_interrupt();

    timer::set_next_trigger();
    println!("2223");
    // load_apps();
    run_first_task();

    // init_heap();
    // heap_test();
    // init_frame_allocator();
    // frame_allocator_test();

    // trap::init();
    // batch::init();
    // batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}



