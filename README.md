# RuPi

Small OS for Raspberry Pi boards built with Rust.
There is no plan, maybe take some input at one point.
Right now it just serves as a shell to get started with OSDev on Raspberry Pi with Rust.

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

objdump - usage command

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