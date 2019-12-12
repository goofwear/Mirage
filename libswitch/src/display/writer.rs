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
//! fn main() {
//!     println!("Hello, world!");
//! }
//! ```

use core::{convert::TryFrom, fmt};

use super::FRAMEBUFFER_ADDRESS;

/// Representations of printable characters.
const GFX_FONT: [[u8; 8]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Char 032 ( )
    [0x00, 0x30, 0x30, 0x18, 0x18, 0x00, 0x0C, 0x00], // Char 033 (!)
    [0x00, 0x22, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00], // Char 034 (")
    [0x00, 0x66, 0x66, 0xFF, 0x66, 0xFF, 0x66, 0x66], // Char 035 (#)
    [0x00, 0x18, 0x7C, 0x06, 0x3C, 0x60, 0x3E, 0x18], // Char 036 ($)
    [0x00, 0x46, 0x66, 0x30, 0x18, 0x0C, 0x66, 0x62], // Char 037 (%)
    [0x00, 0x3C, 0x66, 0x3C, 0x1C, 0xE6, 0x66, 0xFC], // Char 038 (&)
    [0x00, 0x18, 0x0C, 0x06, 0x00, 0x00, 0x00, 0x00], // Char 039 (')
    [0x00, 0x30, 0x18, 0x0C, 0x0C, 0x18, 0x30, 0x00], // Char 040 (()
    [0x00, 0x0C, 0x18, 0x30, 0x30, 0x18, 0x0C, 0x00], // Char 041 ())
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00], // Char 042 (*)
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00], // Char 043 (+)
    [0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x0C, 0x00], // Char 044 (,)
    [0x00, 0x00, 0x00, 0x3E, 0x00, 0x00, 0x00, 0x00], // Char 045 (-)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00], // Char 046 (.)
    [0x00, 0x40, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x00], // Char 047 (/)
    [0x00, 0x3C, 0x66, 0x76, 0x6E, 0x66, 0x3C, 0x00], // Char 048 (0)
    [0x00, 0x18, 0x1C, 0x18, 0x18, 0x18, 0x7E, 0x00], // Char 049 (1)
    [0x00, 0x3C, 0x62, 0x30, 0x0C, 0x06, 0x7E, 0x00], // Char 050 (2)
    [0x00, 0x3C, 0x62, 0x38, 0x60, 0x66, 0x3C, 0x00], // Char 051 (3)
    [0x00, 0x6C, 0x6C, 0x66, 0xFE, 0x60, 0x60, 0x00], // Char 052 (4)
    [0x00, 0x7E, 0x06, 0x7E, 0x60, 0x66, 0x3C, 0x00], // Char 053 (5)
    [0x00, 0x3C, 0x06, 0x3E, 0x66, 0x66, 0x3C, 0x00], // Char 054 (6)
    [0x00, 0x7E, 0x30, 0x30, 0x18, 0x18, 0x18, 0x00], // Char 055 (7)
    [0x00, 0x3C, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00], // Char 056 (8)
    [0x00, 0x3C, 0x66, 0x7C, 0x60, 0x66, 0x3C, 0x00], // Char 057 (9)
    [0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00], // Char 058 (:)
    [0x00, 0x00, 0x18, 0x00, 0x18, 0x18, 0x0C, 0x00], // Char 059 (;)
    [0x00, 0x70, 0x1C, 0x06, 0x06, 0x1C, 0x70, 0x00], // Char 060 (<)
    [0x00, 0x00, 0x3E, 0x00, 0x3E, 0x00, 0x00, 0x00], // Char 061 (=)
    [0x00, 0x0E, 0x38, 0x60, 0x60, 0x38, 0x0E, 0x00], // Char 062 (>)
    [0x00, 0x3C, 0x66, 0x30, 0x18, 0x00, 0x18, 0x00], // Char 063 (?)
    [0x00, 0x3C, 0x66, 0x76, 0x76, 0x06, 0x46, 0x3C], // Char 064 (@)
    [0x00, 0x3C, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00], // Char 065 (A)
    [0x00, 0x3E, 0x66, 0x3E, 0x66, 0x66, 0x3E, 0x00], // Char 066 (B)
    [0x00, 0x3C, 0x66, 0x06, 0x06, 0x66, 0x3C, 0x00], // Char 067 (C)
    [0x00, 0x1E, 0x36, 0x66, 0x66, 0x36, 0x1E, 0x00], // Char 068 (D)
    [0x00, 0x7E, 0x06, 0x1E, 0x06, 0x06, 0x7E, 0x00], // Char 069 (E)
    [0x00, 0x3E, 0x06, 0x1E, 0x06, 0x06, 0x06, 0x00], // Char 070 (F)
    [0x00, 0x3C, 0x66, 0x06, 0x76, 0x66, 0x3C, 0x00], // Char 071 (G)
    [0x00, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00], // Char 072 (H)
    [0x00, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // Char 073 (I)
    [0x00, 0x78, 0x30, 0x30, 0x30, 0x36, 0x1C, 0x00], // Char 074 (J)
    [0x00, 0x66, 0x36, 0x1E, 0x1E, 0x36, 0x66, 0x00], // Char 075 (K)
    [0x00, 0x06, 0x06, 0x06, 0x06, 0x06, 0x7E, 0x00], // Char 076 (L)
    [0x00, 0x46, 0x6E, 0x7E, 0x56, 0x46, 0x46, 0x00], // Char 077 (M)
    [0x00, 0x66, 0x6E, 0x7E, 0x76, 0x66, 0x66, 0x00], // Char 078 (N)
    [0x00, 0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00], // Char 079 (O)
    [0x00, 0x3E, 0x66, 0x3E, 0x06, 0x06, 0x06, 0x00], // Char 080 (P)
    [0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x70, 0x00], // Char 081 (Q)
    [0x00, 0x3E, 0x66, 0x3E, 0x1E, 0x36, 0x66, 0x00], // Char 082 (R)
    [0x00, 0x3C, 0x66, 0x0C, 0x30, 0x66, 0x3C, 0x00], // Char 083 (S)
    [0x00, 0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // Char 084 (T)
    [0x00, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00], // Char 085 (U)
    [0x00, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00], // Char 086 (V)
    [0x00, 0x46, 0x46, 0x56, 0x7E, 0x6E, 0x46, 0x00], // Char 087 (W)
    [0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00], // Char 088 (X)
    [0x00, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00], // Char 089 (Y)
    [0x00, 0x7E, 0x30, 0x18, 0x0C, 0x06, 0x7E, 0x00], // Char 090 (Z)
    [0x00, 0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C], // Char 091 ([)
    [0x00, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x40, 0x00], // Char 092 (\)
    [0x00, 0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C], // Char 093 (])
    [0x00, 0x18, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00], // Char 094 (^)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF], // Char 095 (_)
    [0x00, 0x0C, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00], // Char 096 (`)
    [0x00, 0x00, 0x3C, 0x60, 0x7C, 0x66, 0x7C, 0x00], // Char 097 (a)
    [0x00, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x3E, 0x00], // Char 098 (b)
    [0x00, 0x00, 0x3C, 0x06, 0x06, 0x06, 0x3C, 0x00], // Char 099 (c)
    [0x00, 0x60, 0x60, 0x7C, 0x66, 0x66, 0x7C, 0x00], // Char 100 (d)
    [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x06, 0x3C, 0x00], // Char 101 (e)
    [0x00, 0x38, 0x0C, 0x3E, 0x0C, 0x0C, 0x0C, 0x00], // Char 102 (f)
    [0x00, 0x00, 0x7C, 0x66, 0x7C, 0x40, 0x3C, 0x00], // Char 103 (g)
    [0x00, 0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x00], // Char 104 (h)
    [0x00, 0x18, 0x00, 0x1C, 0x18, 0x18, 0x3C, 0x00], // Char 105 (i)
    [0x00, 0x30, 0x00, 0x30, 0x30, 0x30, 0x1E, 0x00], // Char 106 (j)
    [0x00, 0x06, 0x06, 0x36, 0x1E, 0x36, 0x66, 0x00], // Char 107 (k)
    [0x00, 0x1C, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // Char 108 (l)
    [0x00, 0x00, 0x66, 0xFE, 0xFE, 0xD6, 0xC6, 0x00], // Char 109 (m)
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x66, 0x66, 0x00], // Char 110 (n)
    [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00], // Char 111 (o)
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x00], // Char 112 (p)
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x00], // Char 113 (q)
    [0x00, 0x00, 0x3E, 0x66, 0x06, 0x06, 0x06, 0x00], // Char 114 (r)
    [0x00, 0x00, 0x7C, 0x06, 0x3C, 0x60, 0x3E, 0x00], // Char 115 (s)
    [0x00, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x70, 0x00], // Char 116 (t)
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x7C, 0x00], // Char 117 (u)
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00], // Char 118 (v)
    [0x00, 0x00, 0xC6, 0xD6, 0xFE, 0x7C, 0x6C, 0x00], // Char 119 (w)
    [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00], // Char 120 (x)
    [0x00, 0x00, 0x66, 0x66, 0x7C, 0x60, 0x3C, 0x00], // Char 121 (y)
    [0x00, 0x00, 0x7E, 0x30, 0x18, 0x0C, 0x7E, 0x00], // Char 122 (z)
    [0x00, 0x18, 0x08, 0x08, 0x04, 0x08, 0x08, 0x18], // Char 123 ({)
    [0x00, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08], // Char 124 (|)
    [0x00, 0x0C, 0x08, 0x08, 0x10, 0x08, 0x08, 0x0C], // Char 125 (})
    [0x00, 0x00, 0x00, 0x4C, 0x32, 0x00, 0x00, 0x00], // Char 126 (~)
];

/// The global [`Writer`] instance for the print macros.
///
/// [`Writer`]: struct.Writer.html
const WRITER: Writer = Writer::new();

/// The display height supported by the framebuffer.
const FRAMEBUFFER_HEIGHT: u32 = 1280;
/// The display width supported by the framebuffer.
const FRAMEBUFFER_WIDTH: u32 = 720;
/// The GFX stride for the framebuffer area.
const GFX_STRIDE: u32 = 720;

/// Interface to the framebuffer for drawing contents to the screen.
struct Writer {
    /// A mutable pointer to the framebuffer.
    framebuffer: *mut u32,
    /// The foreground color of the framebuffer area.
    foreground_color: u32,
    /// Whether the background should be filled or not.
    fill_background: bool,
    /// The background color of the framebuffer area.
    background_color: u32,
    x: u32,
    /// The Y coordinate of the cursor.
    y: u32,
}

impl Writer {
    /// Creates a new instance of the [`Writer`] with default values.
    ///
    /// [`Writer`]: struct.Writer.html
    const fn new() -> Self {
        Self {
            framebuffer: FRAMEBUFFER_ADDRESS as *mut u32,
            foreground_color: 0xFFCC_CCCC,
            fill_background: true,
            background_color: 0xFF1B_1B1B,
            x: 0,
            y: 0,
        }
    }

    /// Writes a single character into the framebuffer at the current position.
    /// **Warning:** The character must be in a range between 32 and 126.
    pub fn write_char(&mut self, character: char) -> Result<char, ()> {
        if character == '\n' {
            self.new_line();
            return Ok(character);
        }

        let char_num = u32::try_from(character).expect("Character must fit an u32!");

        // Check if the character is in the allowed range and thus printable.
        if char_num < 32 || char_num > 126 {
            return Err(());
        }

        let char_buf = &GFX_FONT[8 * (char_num as usize - 32)];
        let mut framebuffer =
            self.framebuffer.wrapping_offset((self.x + self.y * GFX_STRIDE) as isize);

        for byte in char_buf.iter() {
            let mut value = byte.clone();

            for _ in 0..8 {
                if value & 1 != 0 {
                    unsafe {
                        framebuffer.write(self.foreground_color);
                    }
                } else if self.fill_background {
                    unsafe {
                        framebuffer.write(self.background_color);
                    }
                }
                value >>= 1;
                framebuffer = framebuffer.wrapping_offset(1);
            }

            framebuffer = framebuffer.wrapping_offset(GFX_STRIDE as isize - 8);
        }

        self.x += 8;

        Ok(character)
    }

    /// Puts a line break at the current position and continues in the next line.
    pub fn new_line(&mut self) {
        self.x = 0;
        self.y += 8;

        if self.y > (FRAMEBUFFER_HEIGHT - 8) {
            self.y = 0;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)
                .expect("Failed to write character to the framebuffer!");
        }

        Ok(())
    }
}

/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::writer::_print(format_args!($($arg)*)));
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::display::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.write_fmt(args).unwrap();
}
