build:
	cargo build --release --target riscv64gc-unknown-none-elf
link:
	rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin
run:
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=./target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

test: build link run