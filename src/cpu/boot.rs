#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/boot.rs"]
mod arch_boot;

#[cfg(target_arch = "arm")]
#[path = "../_arch/armv7a/cpu/boot.rs"]
mod arch_boot;