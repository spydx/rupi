# RuPi

Small OS for Raspberry Pi boards built with Rust.

## Software needed

Expect all rust tools to be installed (rustup and cargo)

```sh
> rustup target add armv7a-none-eabi
```

objdump - usage command

```sh
> objdump -D target/armv7a-none-eabi/debug/rupi | less
```

objcopy

```sh
brew install --cask gcc-arm-embedded
```

## cargo build

cargo rustc -- -C link-arg=--script=./linker.ld

## objcopy command

arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rupi ./kernel7.img

## Raspberry firmware repo

https://github.com/raspberrypi/firmware

Download:

fixup.dat
start.elf
bootcode.bin



Video: https://www.youtube.com/watch?v=jZT8APrzvc4
BCM2837 datasheet: https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf