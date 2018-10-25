#!/bin/sh

bootimage build --verbose --target x86_64-tuxen_os.json;
qemu-system-x86_64 -serial mon:stdio -drive format=raw,file=target/x86_64-tuxen_os/debug/bootimage-TuxenOS.bin -device isa-debug-exit,iobase=0xf4,iosize=0x04;

