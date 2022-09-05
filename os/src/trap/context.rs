use riscv::register::sstatus::{self, Sstatus, SPP};
// 应用程序的上下文，在trap处理那个功能中包括。
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    // 内核地址空间的satp
    pub kernel_satp: usize,
    //当前应用内核地址空间中内核栈栈顶的虚拟位置
    pub kernel_sp: usize,
    //内核中trap handler入口的位置。
    pub trap_handler: usize,
}
impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize, kernel_satp: usize, kernel_sp: usize, trap_handler: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}