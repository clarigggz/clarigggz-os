// SPDX-License-Identifier: MPL-2.0

mod address;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use page_table::{PageTableEntry, PTEFlags};

pub fn init() {
    // Initialize memory management
}
