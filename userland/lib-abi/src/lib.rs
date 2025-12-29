// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub fn syscall(id: usize, args: [usize; 3]) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") id,
            in("a0") args[0],
            in("a1") args[1],
            in("a2") args[2],
            lateout("a0") ret,
        );
    }
    ret
}
