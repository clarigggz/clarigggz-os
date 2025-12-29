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

pub fn ipc_send(to: u64, data: &[u8]) -> usize {
    syscall(1, [to as usize, data.as_ptr() as usize, data.len()])
}

pub fn ipc_recv(to: u64) -> usize {
    syscall(2, [to as usize, 0, 0])
}
