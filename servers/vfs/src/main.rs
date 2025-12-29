// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

mod scheme;

use core::panic::PanicInfo;
use clarigggz_abi::{ipc_recv, ipc_send};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // VFS Server
    loop {
        // Wait for IPC messages
        let msg = ipc_recv(0);
        if msg != usize::MAX {
            // Handle request
        }
    }
}
