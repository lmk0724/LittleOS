use core::arch::asm;
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}
//第二章
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_READ: usize = 63;



pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> ! {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0]);
    panic!("sys_exit never returns!");
}
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}
//第三章
use crate::TaskInfo;
use crate::TimeVal;
pub const SYSCALL_SLEEP: usize = 101;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GETTIMEOFDAY: usize = 169;
pub const SYSCALL_TASK_INFO: usize = 410;

pub fn sys_sleep(sleep_ms: usize) -> isize {
    syscall(SYSCALL_SLEEP, [sleep_ms, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}
pub fn sys_get_time(time: &TimeVal, tz: usize) -> isize {
    syscall(SYSCALL_GETTIMEOFDAY, [time as *const _ as usize, tz, 0])
}
pub fn sys_task_info(info: &TaskInfo) -> isize {
    syscall(SYSCALL_TASK_INFO, [info as *const _ as usize, 0, 0])
}
//第四章

pub const SYSCALL_SET_PRIORITY: usize = 140;
pub const SYSCALL_MUNMAP: usize = 215;
pub const SYSCALL_MMAP: usize = 222;

pub fn sys_set_priority(prio: isize) -> isize {
    syscall(SYSCALL_SET_PRIORITY, [prio as usize, 0, 0])
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    syscall(SYSCALL_MMAP, [start, len, prot])
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    syscall(SYSCALL_MUNMAP, [start, len, 0])
}