use crate::{
    batch::{KernelStack, UserStack},
    config::MAX_APP_NUM,
    sync::UPSafeCell, println,
};

use self::{context::TaskContext, switch::__switch, task::TaskStatus};
use crate::config::{USER_STACK_SIZE,KERNEL_STACK_SIZE,APP_BASE_ADDRESS,APP_SIZE_LIMIT};
use lazy_static::*;
use crate::trap::TrapContext;
mod context;
mod switch;
mod task;
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub context: TaskContext,
    pub task_status: TaskStatus,

    pub kernelStack: KernelStack,
    pub userStack: UserStack,
}
impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            context: TaskContext::zero_init(),
            task_status: TaskStatus::Ready,
            kernelStack: KernelStack {
                data: [0; KERNEL_STACK_SIZE],
            },
            userStack: UserStack {
                data: [0; USER_STACK_SIZE],
            },
        }
    }
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}
lazy_static! {
    static ref TASK_MANAGER: UPSafeCell<TaskManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            println!("num app {}",num_app);
            let mut tasks = [TaskControlBlock::new(); MAX_APP_NUM];
            for i in 0..num_app {
                // println!("init {}", APP_BASE_ADDRESS + i * APP_SIZE_LIMIT);
                let kstack_ptr = tasks[i]
                    .kernelStack
                    .push_context(TrapContext::app_init_context(
                        APP_BASE_ADDRESS + i * APP_SIZE_LIMIT,
                        tasks[i].userStack.get_sp(),
                    ));
                tasks[i].context = TaskContext::goto_restore(kstack_ptr as * const _ as usize);
            }
            let current_task = 2;
            println!("222");
            TaskManager {
                num_app,
                inner: UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task,
                }),
            }
        })
    };
}

impl TaskManager {
    pub fn run_first_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let mut task0 = inner.tasks[inner.current_task];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.context as *const TaskContext;

        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(& mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
    }
    // pub fn find_next_task(&self) -> usize{
    //     let mut inner = self.inner.exclusive_access();
    //     let current = inner.current_task;

    // }
}

pub fn run_first_task(){
    TASK_MANAGER.exclusive_access().run_first_task();
}
// pub use run_first_task;