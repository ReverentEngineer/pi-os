__rpi_dram_start = 0;

PAGE_SIZE = 4096;

ENTRY(_start)

PHDRS {
    segment_boot_core_stack PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0x80000;
    __start = .;
    __text_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text)
    }
    . = ALIGN(PAGE_SIZE); /* align to page size */
    __text_end = .;
 
    __rodata_start = .;
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(PAGE_SIZE); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data)
    }
    . = ALIGN(PAGE_SIZE); /* align to page size */
    __data_end = .;
 
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(PAGE_SIZE); /* align to page size */
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    . = ALIGN(PAGE_SIZE);
    __end = .;
}
