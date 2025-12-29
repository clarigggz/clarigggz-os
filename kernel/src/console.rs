use core::fmt;
use drivers::Ns16550a;
use hal::Uart;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref UART: Mutex<Ns16550a> = {
        let uart = Ns16550a::new(0x1000_0000);
        uart.init();
        Mutex::new(uart)
    };
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    UART.lock().write_fmt(args).unwrap();
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
