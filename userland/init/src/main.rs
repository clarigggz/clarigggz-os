// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use clarigggz_abi::{ipc_send, ipc_recv};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // The first userspace process
    let msg = b"Hello from init!";
    ipc_send(1, msg);
    
    loop {
        let _ = ipc_recv(0);
    }
}
