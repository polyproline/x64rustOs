OUTPUT_FORMAT("elf64-x86-64","elf64-x86-64","elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(kernel_head)
SECTIONS
{
	. = 0xffff800000000000 + 0x100000;
	.text :
	{
		_text = .;
		target/asm/head.o(.text)
		*(.text)
        *(.text .text.*)
		_etext = .;
	}
	. = ALIGN(8);
	.data :
	{
		_data = .;
		*(.data)
		_edata = .;
	}
	.rodata :
	{
		_rodata = .;
		*(.rodata)
		*(.rodata* .rodata.*)
		_erodata = .;
	}
	.bss :
	{
		_bss = .;
		*(.bss)
		*(.bss* .bss.*)
		_ebss = .;
	}
	_end = .;
}
