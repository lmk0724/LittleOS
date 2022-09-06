mod context;


use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::task::{exit_current_run_next, current_trap_cx, current_user_token};
use crate::{task::suspend_current_run_next};
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
fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}
fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}
#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}
#[no_mangle]
pub fn trap_handler() -> ! {
    // let scause = scause::read();
    // let stval = stval::read();
    // let sepc = sepc::read();
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) |
        Trap::Exception(Exception::LoadPageFault) => {
            // println!("[kernel] PageFault in application, kernel killed it.{}",sepc);
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
    trap_return();
    // cx
}
#[no_mangle]
pub fn trap_return() -> ! {
    // println!("trap return during the switch tasks");
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        core::arch::asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
    // println!("trap return conplete during the switch tasks");
}

pub use context::TrapContext;
use riscv::register::sie;

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}