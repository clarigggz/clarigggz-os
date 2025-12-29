// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use clarigggz_abi::syscall;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // The first userspace process
    syscall(1, [0, 0, 0]); // Example syscall
    loop {}
}
