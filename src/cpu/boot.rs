#[cfg(target_arch = "armv7")]
#[path = "../_arch/armv7/cpu/boot.rs"]
mod arch_boot;

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/boot.rs"]
mod arch_boot;