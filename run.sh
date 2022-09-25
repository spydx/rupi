qemu-system-arm \
    -M raspi2b \
    -cpu arm1176 \
    -sd /Users/kenneth.fossen/Downloads/2022-09-06-raspios-bullseye-armhf-lite.img \
    -m 1G \
    -smp 4 \
    -serial stdio \