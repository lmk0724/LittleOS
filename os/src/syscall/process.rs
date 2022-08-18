use crate::println;
use crate::batch::run_next_app;
use crate::task::exit_current_run_next;

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    // run_next_app()
    exit_current_run_next();
    panic!("Unreachable in batch::run_current_app!");
}