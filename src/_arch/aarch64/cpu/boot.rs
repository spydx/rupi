use core::arch::global_asm;

global_asm!(
    include_str!("boot.s"),
    CONST_COREID_MASK = const 0b11
);


// The Rust entry of the `kernel` binary

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}
