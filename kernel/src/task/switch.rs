// SPDX-License-Identifier: MPL-2.0

use core::arch::global_asm;
use super::TaskContext;

global_asm!(include_str!("switch.asm"));

unsafe extern "C" {
    pub fn __switch(
        current_task_cx_ptr: *const TaskContext,
        next_task_cx_ptr: *const TaskContext
    );
}
