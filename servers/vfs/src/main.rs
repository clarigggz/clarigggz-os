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
    // VFS Server
    loop {
        // Wait for IPC messages
    }
}
