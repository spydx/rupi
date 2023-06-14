# RuPi

Small OS for Raspberry Pi boards built with Rust.
There is no plan; maybe take some input at one point.
Right now, it just serves as a shell to start with OSDev on Raspberry Pi with Rust.

Following this guided project: https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

## Details about the Raspberry Pi HW

[Source](https://www.raspberrypi.com/documentation/computers/processors.html)

### Pi2 b

From the webpage:
The Broadcom chip used in the Raspberry Pi 2 Model B. The underlying architecture in BCM2836 is identical to BCM2835. The only significant difference is the removal of the ARM1176JZF-S processor and replacement with a quad-core Cortex-A7 cluster.

### Pi3 b and Pi2b v1.2

This is the Broadcom chip used in the Raspberry Pi 3 Model B, later models of the Raspberry Pi 2 Model B, and the Raspberry Pi Compute Module 3. The underlying architecture of the BCM2837 is identical to the BCM2836. The only significant difference is the replacement of the ARMv7 quad-core cluster with a quad-core ARM Cortex A53 (ARMv8) cluster.

[Docs](./docs/) are downloaded into `./docs/` folder

| Board | Chip |
|---|---|
| RPi1      |   BCM2835 |
| RPi2b     |   BCM2836 |
| RPi3b     |   BCM2837 |
| RPi3b+    |   BCM2837B0 |
| RPi4b     |   BCM2711 |

## Guide Progress

- [x] 00 before_we_start
- [x] 01 wait_forever
- [ ] 02 runtime_init
    In this part we have implemented such as described, but the output from `make qemu` is different from the source project. This might come from we are "building" for RPi2b.
- [ ] 03 hacky_hello_world
- [ ] 04 safe_globals
- [ ] 05 drivers_gpio_uart
- [ ] 06 uart_chainloader
- [ ] 07 timestamps
- [ ] 08 hw_debug_JTAG
- [ ] 09 privilege_level
- [ ] 10 virtual_mem_part1_identity_mapping
- [ ] 11 exceptions_part1_groundwork
- [ ] 12 integrated_testing
- [ ] 13 exceptions_part2_peripheral_IRQs
- [ ] 14 virtual_mem_part2_mmio_remap
- [ ] 15 virtual_mem_part3_precomputed_tables
- [ ] 16 virtual_mem_part4_higher_half_kernel
- [ ] 17 kernel_symbols
- [ ] 18 backtrace
- [ ] 19 kernel_heap

## Software needed

Expect all rust tools to be installed (rustup and cargo)

```sh
rustup target add armv7a-none-eabi
```

```sh
brew install --cask gcc-arm-embedded
```

objdump / objcopy

```sh
brew install --cask gcc-arm-embedded
```

## cargo build

```sh
cargo rustc -- -C link-arg=--script=./linker.ld
```

## objdump usage

```sh
arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/rupi | less
```

## objcopy usage

```sh
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rupi ./kernel7.img
```

## Raspberry firmware repo

Raspberry Pi Firmware Repo [https://github.com/raspberrypi/firmware](https://github.com/raspberrypi/firmware)

Download:

- fixup.dat
- start.elf
- bootcode.bin

## Sources

Source Video: [https://www.youtube.com/watch?v=jZT8APrzvc4](https://www.youtube.com/watch?v=jZT8APrzvc4)

BCM2837 datasheet: [https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf](https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf)

[Linker Scripting](https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Concepts)
[OS Dev on RPi3b+](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
