#![no_std] //don't link the Rust standard library because we can't call a library that doesn't exist on the system
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points because the system won't have a default
//allow a main if running a test
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]//if running a test allow for unused macros and code

use core::panic::PanicInfo;
use TuxenOS::interrupts::PICS;

#[macro_use]
extern crate TuxenOS;

/// This function is called on panic.
#[cfg(not(test))] // only compiled when the test flag is not set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    TuxenOS::hlt_loop();
}


// don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default on linux [ONLY COMPILE ON LINUX SYSTEMS]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut line_text = "";
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

    TuxenOS::gdt::init();
    TuxenOS::interrupts::init_idt();

    // call a breakpoint exception
    //x86_64::instructions::int3();
    //unsafe{ *(0xdeadbeef as *mut u64) = 42; };

    //excuse me while I cause a stack overflow exception
    //fn stack_overflow(){
    //    stack_overflow();
    //}
    //stack_overflow();

    // initialize is unsafe because it could be misconfigured.
    unsafe{ PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); //allow interrupts


    println!("It didn't crash!!!!!!! :)");

    //unsafe { exit_qemu(); }


    TuxenOS::hlt_loop();
}
