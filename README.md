# TuxenOS
The most advanced linux based operating system ever.

# To Use
1. compile the kernel by running ``bootimage build --target x86_64-tuxen_os.json``

2. Run the kernel in QEMU using the command `qemu-system-x86_64 -drive format=raw,file=bootimage-TuxenOS.bin`
    * you can also add `-serial mon:stdio` to see what is passed to the serial ports
    * for exiting the argument `-device isa-debug-exit,iobase=0xf4,iosize=0x04` is required

3. Run `cargo clean` before pushing to clear out clutter
