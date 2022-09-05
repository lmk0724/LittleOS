mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;
mod memeory_set;
pub use heap_allocator::init_heap;
pub use heap_allocator::heap_test;
pub use frame_allocator::{init_frame_allocator,frame_allocator_test};

pub use crate::mm::address::VirtAddr;
use crate::println;

pub use self::memeory_set::KERNEL_SPACE;
pub use memeory_set::MapPermission;



pub fn init() {
    //这三个还是有逻辑上的先后顺序的，
    //先允许rust可以动态申请内存，
    //然后初始化物理帧分配，
    //而后新建内核地址空间。
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}
use alloc::sync::Arc;
pub fn remap_test(){
    let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert_eq!(
        kernel_space.page_table.translate(mid_text.floor()).unwrap().writable(),
        false
    );
    assert_eq!(
        kernel_space.page_table.translate(mid_rodata.floor()).unwrap().writable(),
        false,
    );
    assert_eq!(
        kernel_space.page_table.translate(mid_data.floor()).unwrap().executable(),
        false,
    );

    println!("remap_test passed!");
}
pub use memeory_set::MemorySet;
pub use address::PhysPageNum;
pub use address::PhysAddr;
pub use page_table::translated_byte_buffer;
