.PHONY: boot loader clean bootloader

bootloader:boot loader

boot: boot.bin
	dd if=./boot.bin of=../boot.img bs=512 count=1 conv=notrunc status=progress
loader:loader.bin
	mkdir -p ../disk
	sudo mount ../boot.img ../disk -t vfat -o loop
	sudo cp ./loader.bin ../disk
	sync
	sudo umount ../boot.img

loader.bin:
	nasm loader.asm -o loader.bin

boot.bin:
	nasm boot.asm -o boot.bin

clean:
	rm -rf *.bin *.asm~ Makefile~ loader.bin boot.bin
