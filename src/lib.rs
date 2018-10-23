#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)] // to allow the use because of instability of this resource

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)] // allow the standard library if running tests
extern crate std;

// We need to make these public
#[macro_use]
pub mod vga_buffer;
//pub mod gdt;
pub mod serial;
pub mod interrupts;


//unsafe because it relies on the fact that their is something at that port
// 0xf4 because that port is generally unused and u32 because the iosize is 4 bytes
// 0 for the value because that causes QEMU to exit with exit status
// (0 << 1) | 1 = 1
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

