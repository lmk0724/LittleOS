use crate::config::{MICRO_PER_SEC, PAGE_SIZE};
use crate::mm::{VirtAddr, VirtPageNum, MapPermission};
use crate::println;
// use crate::batch::run_next_app;
use crate::task::{exit_current_run_next, suspend_current_run_next, set_task_info, translate_vpn, contains_key, mmap, m_unmap};
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
    let vaddr = ts as usize;
    let vaddr = VirtAddr(vaddr);
    let page_off = vaddr.page_offset();

    let vpn = vaddr.floor();
    let ppn = translate_vpn(vpn);

    let paddr = ppn.0 << 12 | page_off;
    let _ts = paddr as *mut TimeVal;
    unsafe {
        *_ts = TimeVal{
            sec: usec / MICRO_PER_SEC,
            usec: usec % MICRO_PER_SEC,
        };
    }
    
    // ts.sec = usec / MICRO_PER_SEC;
    // ts.usec = usec%MICRO_PER_SEC;
    0
}
pub fn sys_task_info(ti: *mut TaskInfo)-> isize{
    let vaddr = ti as usize;
    let vaddr = VirtAddr(vaddr);
    let page_off = vaddr.page_offset();

    let vpn = vaddr.floor();
    let ppn = translate_vpn(vpn);

    let paddr = ppn.0 << 12 | page_off;
    let _ti = paddr as *mut TaskInfo;
    set_task_info(_ti);
    0
}

pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize{
    if _port & !0x7 != 0 || _port & 0x7 == 0 || _len <= 0 || _start % PAGE_SIZE !=0{
        return -1;
    }
    let _end = _start + _len;

    let start_vpn = VirtAddr(_start).floor();
    let end_vpn = VirtAddr(_end).ceil();
    
    let mut res = false;
    for i in start_vpn.0..end_vpn.0{
        res = res | contains_key(&VirtPageNum(i)); 
    }
    if res{
        return -1;
    }

    let p = (_port << 1) | 16;
    let permission = MapPermission::from_bits(p as u8).unwrap();
    mmap(VirtAddr(_start) , VirtAddr(_end), permission);
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _len <= 0 || _start % PAGE_SIZE !=0{
        return -1;
    }
    let _end = _start + _len;

    let start_vpn: VirtPageNum = VirtAddr(_start).floor();
    let end_vpn: VirtPageNum = VirtAddr(_end).ceil();
    let mut res = true;
    for i in start_vpn.0..end_vpn.0{
        res = res & contains_key(&VirtPageNum(i));
    }
    if !res{
        return -1;
    }
    m_unmap(start_vpn, end_vpn)
    

    
}
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}