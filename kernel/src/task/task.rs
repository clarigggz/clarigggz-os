// SPDX-License-Identifier: MPL-2.0

use super::context::TaskContext;
use crate::mm::MemorySet;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
    pub id: usize,
    pub memory_set: MemorySet,
}

impl TaskControlBlock {
    pub fn new(id: usize, kstack_ptr: usize) -> Self {
        Self {
            status: TaskStatus::Ready,
            context: TaskContext::goto_trap_return(kstack_ptr),
            id,
            memory_set: MemorySet::new_bare(),
        }
    }
}
