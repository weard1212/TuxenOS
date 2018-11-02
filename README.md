This version is not the most recent. For the most up to date version [view this project on gitlab](https://gitlab.com/weard1212/TuxenOS)
# TuxenOS
The most advanced operating system ever.

# Required Rust Setup
1. Your rust configuration needs to be set to nightly to do this run ``rustup default nightly``

2. The required crates and components can be installed by running ``cargo install bootimage cargo-xbuild && rustup component add rust-src``

# To Compile
1. run ``build.sh``
2. to compile with default parameters ``bootimage run``

# To Compile Manually (Optional)
1. compile the kernel by running ``bootimage build --target x86_64-tuxen_os.json`` on a linux machine

2. Run the kernel in QEMU using the command `qemu-system-x86_64 -drive format=raw,file=target/x86_64-tuxen_os/debug/bootimage-TuxenOS.bin`
    * you can also add `-serial mon:stdio` to see what is passed to the serial ports
    * for exiting the argument `-device isa-debug-exit,iobase=0xf4,iosize=0x04` is required

3. Run `cargo clean` before pushing to clear out clutter

# Testing
1. To run integration tests. `bootimage test`

2. To run the unit tests. `cargo test`

