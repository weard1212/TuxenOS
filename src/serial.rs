// This class is for creating a way to pass data to the host using serial 
// ports 

use uart_16550::SerialPort;
use spin::Mutex;

//lazy_static allows a static method that isn't called at compile time
//but init is called before first use. the reason for 0x3F8 as the port address
// is because it is the standard port number for the first serial interface
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(0x3F8);
        serial_port.init();
        Mutex::new(serial_port)
    };
}

//functions to make the serial port more easily used
pub fn print(args: ::core::fmt::Arguments){
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

//Prints to the host through the serial interface.
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::print(format_args!($($arg)*));
    };
}

// Prints to the host through the serial interface, appending a newline
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}
