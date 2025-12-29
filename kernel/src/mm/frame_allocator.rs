// SPDX-License-Identifier: MPL-2.0

use super::address::PhysPageNum;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    pub fn init(&mut self, start: PhysPageNum, end: PhysPageNum) {
        self.current = start.0;
        self.end = end.0;
    }

    pub fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current < self.end {
            let ppn = self.current;
            self.current += 1;
            Some(ppn.into())
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> = Mutex::new(StackFrameAllocator::new());
}

pub fn frame_alloc() -> Option<PhysPageNum> {
    FRAME_ALLOCATOR.lock().alloc()
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
}
