#!/bin/bash
set -e
cargo build --release
cp target/x86_64-unknown-uefi/release/myos.efi esp/efi/boot/bootx64.efi
qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/OvmfX64/OVMF_CODE.fd -drive format=raw,file=fat:rw:esp
