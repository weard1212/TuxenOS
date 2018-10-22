#!/bin/sh

bootimage build --target x86_64-tuxen_os.json;
qemu-system-x86_64 -serial mon:stdio -drive format=raw,file=target/x86_64-tuxen_os/debug/bootimage-TuxenOS.bin;

