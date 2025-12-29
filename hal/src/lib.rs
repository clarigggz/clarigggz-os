// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub trait Uart {
    fn init(&self);
    fn write_byte(&self, byte: u8);
    fn read_byte(&self) -> Option<u8>;
}

pub trait Display {
    fn flush(&self, buffer: &[u32]);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait Sensor {
    fn read_data(&self) -> [f32; 3];
}
