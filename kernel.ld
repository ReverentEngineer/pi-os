PAGE_SIZE = 4096;

ENTRY(_start)

PHDRS {
    segment_code PT_LOAD FLAGS(5);
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0x80000;
    .text :
    {
        KEEP(*(.text.boot))
	*(.text.kmain)
        *(.text*)
    } :segment_code
    . = ALIGN(PAGE_SIZE); /* align to page size */
    
    .rodata :
    {
        *(.rodata)
    } :segment_code
    . = ALIGN(PAGE_SIZE); /* align to page size */
 
    .data :
    {
        *(.data)
    } :segment_data
    . = ALIGN(PAGE_SIZE); /* align to page size */
 
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
