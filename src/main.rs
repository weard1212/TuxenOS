#![no_std] //don't link the Rust standard library because we can't call a library that doesn't exist on the system
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points because the system won't have a default
//allow a main if running a test
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]//if running a test allow for unused macros and code
#[cfg(test)] // allow the standard library if running tests
extern crate std;
#[cfg(test)]
extern crate array_init;
extern crate uart_16550;
extern crate x86_64;
use core::panic::PanicInfo;
extern crate bootloader_precompiled;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;
#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;

/// This function is called on panic.
#[cfg(not(test))] // only compiled when the test flag is not set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//unsafe because it relies on the fact that their is something at that port
// 0xf4 because that port is generally unused and u32 because the iosize is 4 bytes
// 0 for the value because that causes QEMU to exit with exit status
// (0 << 1) | 1 = 1
pub unsafe fn exit_qemu(){
    use x86_64::instructions::port::Port;
    
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

// don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default on linux [ONLY COMPILE ON LINUX SYSTEMS]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer:WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();*/
    //println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");
    println!("___ _  _ _  _ ____ _  _    ____ ____ ");
    println!(" |  |  |  \\/  |___ |\\ |    |  | [__  ");
    println!(" |  |__| _/\\_ |___ | \\|    |__| ___] ");
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    unsafe { exit_qemu(); }
    
    loop {

    }
}
