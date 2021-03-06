/// This will contain all the code that has to do with printing to the screen.
use volatile::Volatile;
use core::fmt;
use spin::Mutex;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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

// provide a global writer that can be used as an interface from other modules
// lazy_static is a macro that prevents the initialization of static
// methods at compile time and delays them to run time.
lazy_static!{

    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer) },
    });
}
//allow a full color code for both foreground and background
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // makes the ordering like a c struct because rust leaves order undefined
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// the sreen buffer with
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// to actually write to the screen
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// The writer will always write to the last line and shift lines up when a line is full
// column_position field keeps track of the current position in the last row
impl Writer {
    //print a single byte onto the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\t' => self.tab(),
            b'\r' => self.shutdown(),
            b'"' => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // print a string onto the screen one byte at a time.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' | b'\t' | b'\r' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }

    // move every row up one and then clear the last row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // create a tab for the \t character when called.
    fn tab(&mut self){
        self.write_string("    ");
    }

    //backspace function
    fn backspace(&mut self){
        let row = BUFFER_HEIGHT - 1;
        let mut col = 0;
        if self.column_position > 0{
            col = self.column_position - 1;
        }

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: b' ',
            color_code: color_code,
        });
        self.column_position = col;
    }

    fn shutdown(&mut self) {
        self.write_string("Shutting down");
        use power;
        unsafe {power::shutdown();}
    }

    // clear a row by writing a blank space to every character
    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

}

// using a core macro to allow printing of integers and floats
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//making macros to create the functions of print and print ln
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    //prevent deadlocks in our kernel from this function
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}


///
/// SECTION FOR UNIT TESTS
///

#[cfg(test)]//only compile in test
mod test {
    use super::*;//import all modules from the parent

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }


    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

    //make sure that write_byte writes the right character where it is supposed to
    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    // test that the writer is writing the correct characters in
    // the proper places with the right colors
    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}
