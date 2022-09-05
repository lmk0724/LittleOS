

pub const USER_STACK_SIZE: usize = 4096 ;
pub const KERNEL_STACK_SIZE: usize = 4096;
//这里的kernel heap size也得变大，从20000，变为0x300000。
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const MAX_APP_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
pub const CLOCK_FREQ: usize = 12500000;
pub const MAX_SYSCALL_NUM: usize = 500;
pub const MICRO_PER_SEC: usize = 1_000_000;
pub const PAGE_SIZE_BITS: usize = 12;
//需要从80800000改为88000000，要不然初始的kernel都没空间分配完成。
pub const MEMORY_END: usize = 0x88000000;
pub const PAGE_SIZE: usize = 0x1000;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}