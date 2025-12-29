// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // In a real scenario, this would load the kernel from storage
    // and jump to its entry point.
    // For now, it's a placeholder for the M-mode/S-mode transition.
    loop {}
}
