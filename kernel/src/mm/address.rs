// SPDX-License-Identifier: MPL-2.0

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

const PAGE_SIZE: usize = 4096;
const PAGE_SIZE_BITS: usize = 12;

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.0 % PAGE_SIZE, 0);
        Self(v.0 / PAGE_SIZE)
    }
}
impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self { Self(v.0 << PAGE_SIZE_BITS) }
}
