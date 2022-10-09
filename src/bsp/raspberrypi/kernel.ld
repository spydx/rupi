/* entry address for kernel */

__rpi_phys_dram_start_addr = 0;
__rpi_phys_binary_load_addr = 0x80000;

ENTRY(__rpi_phys_binary_load_addr)

/* FLAGS 
 * 4 == R
 * 5 == RX
 * 6 == RW
 */
PHDRS
{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code PT_LOAD FLAGS(5);
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = __rpi_phys_dram_start_addr;

    /* Boot core stack */
    .boot_core_stack (NOLOAD) :
    {
        /* setting growt direction of stack */
        . += __rpi_phys_binary_load_addr;
        __boot_core_stack_end_exclusive = .;
    } : segment_boot_core_stack

    /***
    * CoDe
    * RO data - Global offset table
    ***/
    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* constants read by _start() */
        *(.text._start_rust)    /* rust entry */
        *(.text*)
    } :segment_code

    .rodata : ALIGN(8) { *(.rodata*) }: segment_code

    /** Data + BSS */
    .data : { *(.data*) } : segment_data
    .bss (NOLOAD) : ALIGN(16) 
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } : segment_data

    /** MISC */
    .got : { *(.got*)}
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")
    /DISCARD/ : {*(.comment*)}
}