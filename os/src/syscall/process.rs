use crate::config::MICRO_PER_SEC;
use crate::println;
// use crate::batch::run_next_app;
use crate::task::{exit_current_run_next, suspend_current_run_next, set_task_info};
use crate::timer::{TimeVal, get_time_us, get_time};

use super::TaskInfo;

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    // run_next_app()
    exit_current_run_next();
    panic!("Unreachable in batch::run_current_app!");
}

pub fn sys_yield() -> isize{
    suspend_current_run_next();
    0
}
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize{
    let usec = get_time_us();
    unsafe {
        *ts = TimeVal{
            sec: usec / MICRO_PER_SEC,
            usec: usec%MICRO_PER_SEC,
        };
    }
    
    // ts.sec = usec / MICRO_PER_SEC;
    // ts.usec = usec%MICRO_PER_SEC;
    0
}
pub fn sys_task_info(ti: *mut TaskInfo)-> isize{
    set_task_info(ti);
    0
}