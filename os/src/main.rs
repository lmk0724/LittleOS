#![no_std]
#![no_main]
#![feature(panic_info_message)]
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


use core::arch::global_asm;

use crate::{loader::load_apps, task::run_first_task};

global_asm!(include_str!("entry.asm"));
// fn main() {
//     // println!("Hello, world!");
// }
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("[kernel] Hello, world!");
    
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    load_apps();
    run_first_task();
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



