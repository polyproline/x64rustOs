all:system
	objcopy -I elf64-x86-64 -S -R ".eh_frame" -R ".comment" -O binary target/release/rustos  target/kernel.bin
system: target/asm/head.o
	cargo rustc --release -- -C link-arg=-nostartfiles
target/asm/head.o:	head.S
	mkdir target/asm -p
	gcc -E  head.S -o  target/asm/head.s
	as --64 -o target/asm/head.o target/asm/head.s
clean:
	cargo clean
	rm -rf *.o *.s~ *.s *.S~ *.c~ *.h~ system  Makefile~ Kernel.lds~ kernel.bin 