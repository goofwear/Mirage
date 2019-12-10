//! Framebuffer writer
//!
//! # Description
//!
//! The Framebuffer is a portion of random access memory containing a bitmap that drives a display.
//! The interaction with the Framebuffer is done by the `Writer` struct.
//! It provides basic operations like `write_byte` and `write_string` that will write
//! the given content onto the display. The `Writer` struct also implements
//! the `core::fmt::Write` trait, and thus can be used via the `write` macro.
//! This module also exports the `println` and `print`
//! macro which can be used to print a formatted string onto the display
//! and provides a global instance of the `Writer` which must be used.
//!
//! # Example
//! ```
//! use mirage_libswitch::display;
//!
//! #fn main() {
//!     let writer = display::WRITER;
//!     writer.write_string("Hello ");
//!     println!("World!");
//! #}
//! ```


use core::fmt;

const GFX_FONT: [u8; 760] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Char 032 ( )
	0x00, 0x30, 0x30, 0x18, 0x18, 0x00, 0x0C, 0x00, // Char 033 (!)
	0x00, 0x22, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00, // Char 034 (")
	0x00, 0x66, 0x66, 0xFF, 0x66, 0xFF, 0x66, 0x66, // Char 035 (#)
	0x00, 0x18, 0x7C, 0x06, 0x3C, 0x60, 0x3E, 0x18, // Char 036 ($)
	0x00, 0x46, 0x66, 0x30, 0x18, 0x0C, 0x66, 0x62, // Char 037 (%)
	0x00, 0x3C, 0x66, 0x3C, 0x1C, 0xE6, 0x66, 0xFC, // Char 038 (&)
	0x00, 0x18, 0x0C, 0x06, 0x00, 0x00, 0x00, 0x00, // Char 039 (')
	0x00, 0x30, 0x18, 0x0C, 0x0C, 0x18, 0x30, 0x00, // Char 040 (()
	0x00, 0x0C, 0x18, 0x30, 0x30, 0x18, 0x0C, 0x00, // Char 041 ())
	0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00, // Char 042 (*)
	0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00, // Char 043 (+)
	0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x0C, 0x00, // Char 044 (,)
	0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x00, // Char 045 (-)
	0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, // Char 046 (.)
	0x00, 0x40, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x00, // Char 047 (/)
	0x00, 0x3C, 0x66, 0x76, 0x6E, 0x66, 0x3C, 0x00, // Char 048 (0)
	0x00, 0x18, 0x1C, 0x18, 0x18, 0x18, 0x7E, 0x00, // Char 049 (1)
	0x00, 0x3C, 0x62, 0x30, 0x0C, 0x06, 0x7E, 0x00, // Char 050 (2)
	0x00, 0x3C, 0x62, 0x38, 0x60, 0x66, 0x3C, 0x00, // Char 051 (3)
	0x00, 0x6C, 0x6C, 0x66, 0xFE, 0x60, 0x60, 0x00, // Char 052 (4)
	0x00, 0x7E, 0x06, 0x7E, 0x60, 0x66, 0x3C, 0x00, // Char 053 (5)
	0x00, 0x3C, 0x06, 0x3E, 0x66, 0x66, 0x3C, 0x00, // Char 054 (6)
	0x00, 0x7E, 0x30, 0x30, 0x18, 0x18, 0x18, 0x00, // Char 055 (7)
	0x00, 0x3C, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00, // Char 056 (8)
	0x00, 0x3C, 0x66, 0x7C, 0x60, 0x66, 0x3C, 0x00, // Char 057 (9)
	0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, // Char 058 (:)
	0x00, 0x00, 0x18, 0x00, 0x18, 0x18, 0x0C, 0x00, // Char 059 (;)
	0x00, 0x70, 0x1C, 0x06, 0x06, 0x1C, 0x70, 0x00, // Char 060 (<)
	0x00, 0x00, 0x3E, 0x00, 0x3E, 0x00, 0x00, 0x00, // Char 061 (=)
	0x00, 0x0E, 0x38, 0x60, 0x60, 0x38, 0x0E, 0x00, // Char 062 (>)
	0x00, 0x3C, 0x66, 0x30, 0x18, 0x00, 0x18, 0x00, // Char 063 (?)
	0x00, 0x3C, 0x66, 0x76, 0x76, 0x06, 0x46, 0x3C, // Char 064 (@)
	0x00, 0x3C, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00, // Char 065 (A)
	0x00, 0x3E, 0x66, 0x3E, 0x66, 0x66, 0x3E, 0x00, // Char 066 (B)
	0x00, 0x3C, 0x66, 0x06, 0x06, 0x66, 0x3C, 0x00, // Char 067 (C)
	0x00, 0x1E, 0x36, 0x66, 0x66, 0x36, 0x1E, 0x00, // Char 068 (D)
	0x00, 0x7E, 0x06, 0x1E, 0x06, 0x06, 0x7E, 0x00, // Char 069 (E)
	0x00, 0x3E, 0x06, 0x1E, 0x06, 0x06, 0x06, 0x00, // Char 070 (F)
	0x00, 0x3C, 0x66, 0x06, 0x76, 0x66, 0x3C, 0x00, // Char 071 (G)
	0x00, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00, // Char 072 (H)
	0x00, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, // Char 073 (I)
	0x00, 0x78, 0x30, 0x30, 0x30, 0x36, 0x1C, 0x00, // Char 074 (J)
	0x00, 0x66, 0x36, 0x1E, 0x1E, 0x36, 0x66, 0x00, // Char 075 (K)
	0x00, 0x06, 0x06, 0x06, 0x06, 0x06, 0x7E, 0x00, // Char 076 (L)
	0x00, 0x46, 0x6E, 0x7E, 0x56, 0x46, 0x46, 0x00, // Char 077 (M)
	0x00, 0x66, 0x6E, 0x7E, 0x76, 0x66, 0x66, 0x00, // Char 078 (N)
	0x00, 0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00, // Char 079 (O)
	0x00, 0x3E, 0x66, 0x3E, 0x06, 0x06, 0x06, 0x00, // Char 080 (P)
	0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x70, 0x00, // Char 081 (Q)
	0x00, 0x3E, 0x66, 0x3E, 0x1E, 0x36, 0x66, 0x00, // Char 082 (R)
	0x00, 0x3C, 0x66, 0x0C, 0x30, 0x66, 0x3C, 0x00, // Char 083 (S)
	0x00, 0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, // Char 084 (T)
	0x00, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00, // Char 085 (U)
	0x00, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00, // Char 086 (V)
	0x00, 0x46, 0x46, 0x56, 0x7E, 0x6E, 0x46, 0x00, // Char 087 (W)
	0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00, // Char 088 (X)
	0x00, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00, // Char 089 (Y)
	0x00, 0x7E, 0x30, 0x18, 0x0C, 0x06, 0x7E, 0x00, // Char 090 (Z)
	0x00, 0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, // Char 091 ([)
	0x00, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x40, 0x00, // Char 092 (\)
	0x00, 0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, // Char 093 (])
	0x00, 0x18, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00, // Char 094 (^)
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // Char 095 (_)
	0x00, 0x0C, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, // Char 096 (`)
	0x00, 0x00, 0x3C, 0x60, 0x7C, 0x66, 0x7C, 0x00, // Char 097 (a)
	0x00, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x3E, 0x00, // Char 098 (b)
	0x00, 0x00, 0x3C, 0x06, 0x06, 0x06, 0x3C, 0x00, // Char 099 (c)
	0x00, 0x60, 0x60, 0x7C, 0x66, 0x66, 0x7C, 0x00, // Char 100 (d)
	0x00, 0x00, 0x3C, 0x66, 0x7E, 0x06, 0x3C, 0x00, // Char 101 (e)
	0x00, 0x38, 0x0C, 0x3E, 0x0C, 0x0C, 0x0C, 0x00, // Char 102 (f)
	0x00, 0x00, 0x7C, 0x66, 0x7C, 0x40, 0x3C, 0x00, // Char 103 (g)
	0x00, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x00, // Char 104 (h)
	0x00, 0x18, 0x00, 0x1C, 0x18, 0x18, 0x3C, 0x00, // Char 105 (i)
	0x00, 0x30, 0x00, 0x30, 0x30, 0x30, 0x1E, 0x00, // Char 106 (j)
	0x00, 0x06, 0x06, 0x36, 0x1E, 0x36, 0x66, 0x00, // Char 107 (k)
	0x00, 0x1C, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, // Char 108 (l)
	0x00, 0x00, 0x66, 0xFE, 0xFE, 0xD6, 0xC6, 0x00, // Char 109 (m)
	0x00, 0x00, 0x3E, 0x66, 0x66, 0x66, 0x66, 0x00, // Char 110 (n)
	0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00, // Char 111 (o)
	0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x00, // Char 112 (p)
	0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x00, // Char 113 (q)
	0x00, 0x00, 0x3E, 0x66, 0x06, 0x06, 0x06, 0x00, // Char 114 (r)
	0x00, 0x00, 0x7C, 0x06, 0x3C, 0x60, 0x3E, 0x00, // Char 115 (s)
	0x00, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x70, 0x00, // Char 116 (t)
	0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x7C, 0x00, // Char 117 (u)
	0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00, // Char 118 (v)
	0x00, 0x00, 0xC6, 0xD6, 0xFE, 0x7C, 0x6C, 0x00, // Char 119 (w)
	0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00, // Char 120 (x)
	0x00, 0x00, 0x66, 0x66, 0x7C, 0x60, 0x3C, 0x00, // Char 121 (y)
	0x00, 0x00, 0x7E, 0x30, 0x18, 0x0C, 0x7E, 0x00, // Char 122 (z)
	0x00, 0x18, 0x08, 0x08, 0x04, 0x08, 0x08, 0x18, // Char 123 ({)
	0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, // Char 124 (|)
	0x00, 0x0C, 0x08, 0x08, 0x10, 0x08, 0x08, 0x0C, // Char 125 (})
	0x00, 0x00, 0x00, 0x4C, 0x32, 0x00, 0x00, 0x00  // Char 126 (~)
];

const FRAMEBUFFER_HEIGHT: usize = 1280;
const FRAMEBUFFER_WIDTH: usize = 720;
const GFX_STRIDE: usize = 720;

// TODO: Add global constant that compiles!
/*
 *pub const WRITER: Writer = Writer {
 *    buffer: unsafe { &mut *(crate::display::FRAMEBUFFER_ADDRESS as *mut Framebuffer) },
 *    foreground_color: 0xFFCCCCCC,
 *    fill_background: false,
 *    background_color: 0xFF1B1B1B,
 *    x: 0,
 *    y: 0
 *};
 */

#[repr(transparent)]
struct Framebuffer {
    pixels: [u32; FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT],
}

pub struct Writer {
    buffer: &'static mut Framebuffer,
    foreground_color: u32,
    fill_background: bool,
    background_color: u32,
    x: u32,
    y: u32,
}

impl Writer {

    /// Writes a single byte into the framebuffer at the current position.
    /// **Warning:** The byte must be between 32 and 126
    pub fn write_byte(&mut self, byte: u8) {
        if byte == '\n' as u8 {
            self.new_line();
            return
        }

        if byte < 32 || byte > 126 {
            return
        }

        let char_buf = &GFX_FONT[(8usize * (byte as usize - 32usize))..];
        let mut fb = &self.buffer.pixels[(self.x as usize + self.y as usize * GFX_STRIDE)..];

        // TODO: Fix E0594 in line 178, 180, 182
        for _ in 0..8 {
            let v = &char_buf[1..];
            for _ in 0..8 {
            if v[0] & 1 > 0 {
                fb[0] = self.foreground_color;
                } else if self.fill_background {
                    fb[0] = self.background_color;
                }
                v[0] >>= 1;
                fb = &fb[1..];
            }
            self.x += GFX_STRIDE as u32 - 8;
        }
        self.x += 8;
    }

    /// Writes a sequence of bytes into the framebuffer.
    pub fn write_string(&mut self, string: &str) {
        for byte in string.as_bytes().iter() {
            self.write_byte(byte.clone());
        }
    }

    /// Jumps one line down and continues to write there.
    pub fn new_line(&mut self) {
        self.x = 0;
        self.y += 8;
        if self.y > (FRAMEBUFFER_HEIGHT as u32 - 8) {
            self.y = 0;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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
pub fn _print(_args: fmt::Arguments) {
    // TODO: uncomment this line as soon as a global instance is implemented.
    // use core::fmt::Write;
    // WRITER.write_fmt(args).unwrap();
}
