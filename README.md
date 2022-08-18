# LittleOS

项目在master分支下面，os目录是操作系统内核，user是用户程序。
### 使用方法
* 在user目录下需要手动执行python3 build.py，会生成用户程序的bin文件。
* 而后在os目录下，cargo build --release，这会调用build.rs文件，生成用户程序链接到内核的汇编文件。
* 继续在os目录下执行下面的两条指令，裁剪os，在qemu上运行os。
```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

qemu-system-riscv64 -machine virt -nographic -bios bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

### ch3的进度
主要是实现了TaskControlBlock，这个类与rcore-tutorial原始的代码不同，将内核栈，用户栈全部添加进去了。然后实现了TaskManager类。

目前的代码仅限于run_first_task这个功能。还没有实现时间片轮转的功能。