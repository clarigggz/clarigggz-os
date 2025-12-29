// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hal::Uart;
use volatile::Volatile;

pub struct Ns16550a {
    base: usize,
}

impl Ns16550a {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    fn ptr(&self, offset: usize) -> *mut u8 {
        (self.base + offset) as *mut u8
    }
}

impl Uart for Ns16550a {
    fn init(&self) {
        // Simple initialization for QEMU virt UART
        unsafe {
            self.ptr(3).write_volatile(0b11); // LCR: 8n1
            self.ptr(2).write_volatile(0b1);  // FCR: enable FIFO
            self.ptr(1).write_volatile(0b1);  // IER: enable RX interrupt
        }
    }

    fn putc(&self, c: u8) {
        unsafe {
            // Wait for THR empty
            while (self.ptr(5).read_volatile() & (1 << 5)) == 0 {}
            self.ptr(0).write_volatile(c);
        }
    }

    fn getc(&self) -> Option<u8> {
        unsafe {
            if (self.ptr(5).read_volatile() & 1) != 0 {
                Some(self.ptr(0).read_volatile())
            } else {
                None
            }
        }
    }
}

pub fn init() {
    // Initialize drivers
}
