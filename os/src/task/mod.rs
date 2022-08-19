use crate::{
    batch::{KernelStack, UserStack},
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM, CLOCK_FREQ, MICRO_PER_SEC},
    sync::UPSafeCell, println, timer::get_time, syscall::TaskInfo,
};

use self::{context::TaskContext, switch::__switch};
use crate::config::{USER_STACK_SIZE,KERNEL_STACK_SIZE,APP_BASE_ADDRESS,APP_SIZE_LIMIT};
use lazy_static::*;
use crate::trap::TrapContext;
mod context;
mod switch;
// mod task;
// pub use task::TaskStatus;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running, // 正在运行
    Exited, // 已退出
}

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub context: TaskContext,
    pub task_status: TaskStatus,

    pub kernelStack: KernelStack,
    pub userStack: UserStack,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub start_time: usize,
}
impl TaskControlBlock {
    pub fn new() -> Self {
        Self {
            context: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            kernelStack: KernelStack {
                data: [0; KERNEL_STACK_SIZE],
            },
            userStack: UserStack {
                data: [0; USER_STACK_SIZE],
            },
            syscall_times: [0u32;MAX_SYSCALL_NUM],
            start_time: 0,
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
    static ref TASK_MANAGER: TaskManager = unsafe {
        {
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            println!("num app {}",num_app);
            let mut tasks = [TaskControlBlock::new(); MAX_APP_NUM];
            println!("333");
            for i in 0..num_app {
                // println!("init {}", APP_BASE_ADDRESS + i * APP_SIZE_LIMIT);
                let kstack_ptr = tasks[i]
                    .kernelStack
                    .push_context(TrapContext::app_init_context(
                        APP_BASE_ADDRESS + i * APP_SIZE_LIMIT,
                        tasks[i].userStack.get_sp(),
                    ));
                tasks[i].context = TaskContext::goto_restore(kstack_ptr as * const _ as usize);
                tasks[i].start_time = get_time();
                tasks[i].task_status = TaskStatus::Ready;
                println!("444");
            }
            let current_task = 0;
            println!("222");
            TaskManager {
                num_app,
                inner: UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task,
                }),
            }
        }
    };
}

impl TaskManager {
    pub fn run_first_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let mut task0 = inner.tasks[inner.current_task];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.context as *const TaskContext;

        let mut _unused = TaskContext::zero_init();
        //这部分需要研究
        drop(inner);

        unsafe {
            __switch(& mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
    }
    pub fn find_next_task(&self) -> Option<usize>{
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        for i in current+1..current+self.num_app+1{
            let index = i%self.num_app;
            let task = inner.tasks[index];
            if task.task_status == TaskStatus::Ready{
                return Some(index);
            }
        }
        return None;

    }
    pub fn run_next_task(&self){
        if let Some(index) = self.find_next_task(){
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            let mut current_task_cx_ptr = & mut inner.tasks[current].context as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[index].context as * const TaskContext;
            inner.tasks[index].task_status = TaskStatus::Running;
            inner.current_task = index;

            drop(inner);

            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        }else{
            println!("no task in ready");
        }
        
    }
    pub fn mark_current_ready(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
    pub fn mark_current_exit(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
    pub fn update_syscall_arr(&self, sys_id:usize){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[sys_id] += 1;
    }
    pub fn set_task_info(&self, ti: *mut TaskInfo){
        // ti.status = TaskStatus::Running;
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let sys_arr = inner.tasks[current].syscall_times.clone();
        // ti.syscall_times = sys_arr;
        let current_time = get_time();
        let start_time = inner.tasks[current].start_time;
        let time = (current_time-start_time)/CLOCK_FREQ * MICRO_PER_SEC /1000;
        unsafe{
            *ti = TaskInfo{
                status : TaskStatus::Running,
                syscall_times : sys_arr,
                time: time,
            };
        }

    }
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}
pub fn mark_exit(){
    TASK_MANAGER.mark_current_exit();
}
pub fn mark_ready(){
    TASK_MANAGER.mark_current_ready();
}
pub fn run_next_task(){
    TASK_MANAGER.run_next_task();
}
pub fn suspend_current_run_next(){
    mark_ready();
    run_next_task();
}
pub fn exit_current_run_next(){
    mark_exit();
    run_next_task();
}
pub fn update_syscall_arr(sys_id: usize){
    TASK_MANAGER.update_syscall_arr(sys_id);
}
pub fn set_task_info(ti: *mut TaskInfo){
    TASK_MANAGER.set_task_info(ti);
}

