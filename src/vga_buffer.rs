use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::unsafe_ptr::UnsafePointer;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::Yellow, Color::Black)
    }
}

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    base_addr: UnsafePointer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.write_row_col(
                    row,
                    col,
                    ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    },
                );
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write(&self, location: usize, byte: u8, color_code: ColorCode) {
        let mut addr = self.base_addr.add(location);
        unsafe {
            addr.write_volatile(byte);
            addr = addr.add(1);
            addr.write_volatile(color_code.0);
        }
    }

    fn write_row_col(&self, row: usize, col: usize, screen_char: ScreenChar) {
        let location = 2 * (row * BUFFER_WIDTH + col);
        self.write(
            location,
            screen_char.ascii_character,
            screen_char.color_code,
        );
    }

    fn new_line(&mut self) {
        let mut dest = self.base_addr;
        let mut src = self.base_addr.add(2 * BUFFER_WIDTH);
        let last = 2 * (BUFFER_HEIGHT - 1) * BUFFER_WIDTH;
        for _ in 0..last {
            unsafe {
                let byte: u8 = src.read_volatile();
                dest.write_volatile(byte);
                src = src.add(1);
                dest = dest.add(1);
            }
        }

        // clear last line
        for _ in 0..2 * BUFFER_WIDTH {
            unsafe {
                dest.write_volatile::<u8>(0);
                dest = dest.add(1);
            }
        }

        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: Default::default(),
        base_addr: UnsafePointer::new(0xb8000),
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
