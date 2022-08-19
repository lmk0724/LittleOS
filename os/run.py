import os
os.system("cargo clean")
os.system("cargo build --release")
os.system("rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin")
os.system("qemu-system-riscv64 -machine virt -nographic -bios bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000")
