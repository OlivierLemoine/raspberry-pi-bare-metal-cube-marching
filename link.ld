SECTIONS
{
    /*. = 0x0;
    .exception_vector :
    {
        . = 0x280;
        KEEP(*(.SP_ELx_IRQ))
    }*/

    . = 0x80000;
    __start_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    PROVIDE(_data = .);

    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    }

    . = ALIGN(16);
    __start_end = .;

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
__bss_size = (__bss_end - __bss_start) >> 3;
__start_size = __start_end - __start_start;
