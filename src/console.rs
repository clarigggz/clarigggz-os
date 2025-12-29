use core::fmt;

pub struct Console;

impl Console {
    pub fn write_byte(&self, byte: u8) {
        let uart = 0x4000_0000 as *mut u8;
        unsafe {
            uart.write_volatile(byte);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut console = Console;
    console.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
