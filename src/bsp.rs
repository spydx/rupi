#[cfg(any(feature = "bsp_rpi2b", feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi2b", feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspberrypi::*;