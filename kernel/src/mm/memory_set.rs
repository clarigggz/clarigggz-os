// SPDX-License-Identifier: MPL-2.0

use super::page_table::{PageTableEntry, PTEFlags};
use super::address::{PhysPageNum, VirtPageNum, VirtAddr, PhysAddr};
use super::frame_allocator::{frame_alloc, frame_dealloc};
use alloc::vec::Vec;
use alloc::vec;

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<PhysPageNum>,
}

impl PageTable {
    pub fn new() -> Self {
        let ppn = frame_alloc().unwrap();
        Self {
            root_ppn: ppn,
            frames: vec![ppn],
        }
    }

    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = [
            (vpn.0 >> 18) & 0x1ff,
            (vpn.0 >> 9) & 0x1ff,
            vpn.0 & 0x1ff,
        ];
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte_ptr = (PhysAddr::from(ppn).0 + idx * 8) as *mut PageTableEntry;
            let pte = unsafe { &mut *pte_ptr };
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is already mapped", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }
}
