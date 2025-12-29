// SPDX-License-Identifier: MPL-2.0

#![no_std]
#![no_main]

mod console;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("Clarigggz OS Booting...");
    println!("Architecture: RISC-V 64-bit");
    println!("Target: RV64GCV");
    
    loop {}
}
