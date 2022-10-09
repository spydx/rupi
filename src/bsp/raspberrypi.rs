//! Top-level BSP file for the Raspberry Pi 2, 3 and 4.
#[cfg(target_arch = "aarch64")]
#[path = "raspberrypi/cpu.rs"]
pub mod cpu;

#[cfg(target_arch = "arm")]
#[path = "rpi2b/cpu.rs"]
pub mod cpu;