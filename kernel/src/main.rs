// SPDX-License-Identifier: MPL-2.0

#![no_std]
#![no_main]

extern crate alloc;

mod console;
mod ipc;
mod trap;

use core::arch::global_asm;
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;

global_asm!(include_str!("entry.asm"));

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // Initialize heap
    unsafe {
        HEAP_ALLOCATOR.lock().init(core::ptr::addr_of_mut!(HEAP_SPACE) as usize, HEAP_SIZE);
    }

    println!("Clarigggz OS Booting...");
    println!("Architecture: RISC-V 64-bit");
    println!("Target: RV64GCV");

    trap::init();
    
    loop {}
}
