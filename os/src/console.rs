// os/src/console.rs
use core::fmt;
use core::fmt::{Arguments, Write};
use crate::sbi::console_putchar;


struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

//rust 的语法看起来有点诡异啊🤔
#[allow(unused_macros)]
macro_rules! print {
    ($fmt:literal $(,$($arg:tt)+)?) => {
         $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}