#!/bin/zsh

# clearing target and outfolder
rm -rf target
rm -rf out
mkdir out

echo "Build target"
cargo rustc -- -C link-arg=--script=./linker.ld

echo "Create kernel image"
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rupi ./out/kernel7.img

echo "Fetch Raspberry Pi boot files"
wget -P ./out https://github.com/raspberrypi/firmware/raw/master/boot/bootcode.bin
wget -P ./out https://github.com/raspberrypi/firmware/raw/master/boot/start.elf
wget -P ./out https://github.com/raspberrypi/firmware/raw/master/boot/fixup.dat

cp -R ./out/ /Volumes/RUPI