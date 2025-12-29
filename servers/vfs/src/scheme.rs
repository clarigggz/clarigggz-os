// SPDX-License-Identifier: Apache-2.0

pub trait Scheme {
    fn open(&mut self, path: &str, flags: usize) -> Result<usize, ()>;
    fn read(&mut self, id: usize, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, id: usize, buf: &[u8]) -> Result<usize, ()>;
    fn close(&mut self, id: usize) -> Result<(), ()>;
}
