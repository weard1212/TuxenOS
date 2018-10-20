#![no_std] //don't link the Rust standard library because we can't call a library that doesn't exist on the system
#![no_main] // disable all Rust-level entry points because the system won't have a default

use core::panic::PanicInfo;
extern crate bootloader_precompiled;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;
#[macro_use]
mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default on linux [ONLY COMPILE ON LINUX SYSTEMS]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer:WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();*/
    println!("Hello World{}", "!");

    loop {

    }
}
