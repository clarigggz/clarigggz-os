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

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is not mapped", vpn);
        *pte = PageTableEntry::empty();
    }
}

pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
}

pub struct MapArea {
    vpn_range: (VirtPageNum, VirtPageNum),
    flags: PTEFlags,
}

use xmas_elf::ElfFile;

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn push(&mut self, start_va: VirtAddr, end_va: VirtAddr, flags: PTEFlags) {
        let start_vpn = VirtPageNum(start_va.0 / 4096);
        let end_vpn = VirtPageNum((end_va.0 + 4095) / 4096);
        for vpn in start_vpn.0..end_vpn.0 {
            let frame = frame_alloc().unwrap();
            self.page_table.map(VirtPageNum(vpn), frame, flags);
        }
        self.areas.push(MapArea {
            vpn_range: (start_vpn, end_vpn),
            flags,
        });
    }

    pub fn from_elf(elf_data: &[u8]) -> (Self, usize) {
        let mut memory_set = Self::new_bare();
        let elf = ElfFile::new(elf_data).expect("Invalid ELF file");
        let header = elf.header;
        let magic = header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "Invalid ELF magic");
        
        let ph_count = header.pt2.ph_count();
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut flags = PTEFlags::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() { flags |= PTEFlags::R; }
                if ph_flags.is_write() { flags |= PTEFlags::W; }
                if ph_flags.is_execute() { flags |= PTEFlags::X; }
                
                memory_set.push(start_va, end_va, flags);
                
                // Copy data
                let data = &elf_data[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize];
                for (j, byte) in data.iter().enumerate() {
                    let va = start_va.0 + j;
                    let vpn = VirtPageNum(va / 4096);
                    let offset = va % 4096;
                    let pte = memory_set.page_table.find_pte_create(vpn).unwrap();
                    let pa = PhysAddr::from(pte.ppn()).0 + offset;
                    unsafe {
                        (pa as *mut u8).write_volatile(*byte);
                    }
                }
            }
        }
        (memory_set, header.pt2.entry_point() as usize)
    }
}
