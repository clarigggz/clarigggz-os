// SPDX-License-Identifier: MPL-2.0

mod context;
mod task;
mod switch;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
pub use task::{TaskControlBlock, TaskStatus};
pub use context::TaskContext;

pub struct TaskManager {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_task: 0,
        }
    }

    pub fn add_task(&mut self, task: TaskControlBlock) {
        self.tasks.push(task);
    }

    pub fn run_first_task(&mut self) -> ! {
        let task = &mut self.tasks[0];
        task.status = TaskStatus::Running;
        
        // Switch to task's page table
        unsafe {
            riscv::register::satp::write(task.memory_set.page_table.token());
            core::arch::asm!("sfence.vma");
        }

        let next_task_cx_ptr = &task.context as *const TaskContext;
        let _unused = TaskContext::zero();
        unsafe {
            switch::__switch(
                &_unused as *const TaskContext,
                next_task_cx_ptr,
            );
        }
        panic!("unreachable in run_first_task!");
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn run_first_task() -> ! {
    TASK_MANAGER.lock().run_first_task();
}

pub fn suspend_current_and_run_next() {
    let mut manager = TASK_MANAGER.lock();
    let current = manager.current_task;
    let next = (current + 1) % manager.tasks.len();
    
    if current == next { return; }

    manager.tasks[current].status = TaskStatus::Ready;
    manager.tasks[next].status = TaskStatus::Running;
    manager.current_task = next;

    let current_cx_ptr = &manager.tasks[current].context as *const TaskContext;
    let next_cx_ptr = &manager.tasks[next].context as *const TaskContext;
    let next_token = manager.tasks[next].memory_set.page_table.token();

    drop(manager);

    unsafe {
        riscv::register::satp::write(next_token);
        core::arch::asm!("sfence.vma");
        switch::__switch(current_cx_ptr, next_cx_ptr);
    }
}
