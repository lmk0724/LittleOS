mod context;
use crate::task::exit_current_run_next;
use crate::{batch::run_next_app, task::suspend_current_run_next};
use crate::syscall::syscall;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap,Interrupt},
    stval, stvec, sepc,
};
use crate::timer::set_next_trigger;
use crate::println;

core::arch::global_asm!(include_str!("trap.S"));
// 设置Trap处理函数的地址。
pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    let sepc = sepc::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.{}",sepc);
            exit_current_run_next()
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) =>{
            // println!("time interrupt");
            set_next_trigger();
            suspend_current_run_next();
        }
        // Trap::Interrupt(Interrupt::SupervisorTimer) => {
        //     set_next_trigger();
        //     println!("timer interrupt");
        // }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}
pub use context::TrapContext;
use riscv::register::sie;

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}