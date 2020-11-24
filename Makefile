.PHONY: run clean

run: 
	bochs -f bochsrc

clean:
	make -C bootloader clean
	make -C rustos clean
	rm ./disk
