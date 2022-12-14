#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

#[cfg(target_arch = "arm")]
#[path = "_arch/armv7a/cpu.rs"]
mod arch_cpu;

mod boot;

// Architecture reexports

pub use arch_cpu::wait_forever;