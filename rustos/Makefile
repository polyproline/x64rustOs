.PHONY:  clean kernel

kernel:target/kernel.bin
	mkdir -p ../disk
	sudo mount ../boot.img ../disk -t vfat -o loop
	sudo cp ./target/kernel.bin ../disk
	sync
	sudo umount ../boot.img

target/kernel.bin:target/os/release/rustos 
	objcopy -I elf64-x86-64 -S -R ".eh_frame" -R ".comment" -O binary target/os/release/rustos  target/kernel.bin

target/os/release/rustos: target/asm/head.o os.json Cargo.toml target/asm/head.o
	export RUSTFLAGS="-C code-model=large -C relocation-model=static" && \
	cargo build --target=os.json --release
target/asm/head.o:	head.S
	mkdir target/asm -p
	gcc -E  head.S -o  target/asm/head.s
	as --64 -o target/asm/head.o target/asm/head.s
clean:
	cargo clean
	rm -rf *.o *.s~ *.s *.S~ *.c~ *.h~ system  Makefile~ Kernel.lds~