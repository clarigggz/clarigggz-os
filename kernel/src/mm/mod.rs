// SPDX-License-Identifier: MPL-2.0

mod address;
mod page_table;
mod frame_allocator;
mod memory_set;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use page_table::{PageTableEntry, PTEFlags};
pub use frame_allocator::{frame_alloc, frame_dealloc};
pub use memory_set::PageTable;

pub fn init() {
    // Initialize memory management
    // In a real scenario, we'd get the available memory from the bootloader/DTB
    // For now, we'll assume a fixed range for the frame allocator
    let start_ppn = PhysPageNum(0x8020_0000 / 4096 + 1024); // After kernel
    let end_ppn = PhysPageNum(0x8800_0000 / 4096); // 128MB
    frame_allocator::FRAME_ALLOCATOR.lock().init(start_ppn, end_ppn);
}
