# RuPi

Small OS for Raspberry Pi boards built with Rust.
There is no plan, maybe take some input at one point.
Right now it just serves as a shell to get started with OSDev on Raspberry Pi with Rust.

## Guide Progress

- [ ] 00 before_we_start
- [ ] 01 wait_forever
- [ ] 02 runtime_init
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

[OS Dev on RPi2b](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)