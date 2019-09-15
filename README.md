# rpi3-rust-workspace

Rust workspace for RPI3 bare metal things

Inspired by [rust-raspi3-OS-tutorials](https://github.com/rust-embedded/rust-raspi3-OS-tutorials).

## Building

```rust
cargo xbuild
```

Copy elf to binary:

```bash
cargo objcopy -- -O binary target/$(TARGET)/release/img /tmp/img.bin
```

## Simulating

```bash
# For output on UART1
qemu-system-aarch64 -M raspi3 -nographic -serial null -serial mon:stdio -kernel /path/to/binary
```

## U-boot

Using 64 bit U-boot:

```bash
git clone --depth 1 https://github.com/u-boot/u-boot.git u-boot
ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- make rpi_3_defconfig
ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- make
```

```bash
U-Boot> version
U-Boot 2018.11-g208ecba (Nov 14 2018 - 13:17:50 -0800)

aarch64-linux-gnu-gcc (Linaro GCC 7.3-2018.05) 7.3.1 20180425 [linaro-7.3-2018.05 revision d29120a424ec
fbc167ef90065c0eeb7f91977701]
GNU ld (Linaro_Binutils-2018.05) 2.28.2.20170706
```

Environment:

```bash
setenv imgname img.bin

# Normally for bare metal
#setenv loadaddr 0x80000

# Put it somewhere else, so we don't overwrite u-boot
setenv loadaddr 0x01000000

# Disable data cache because u-boot turns it on
setenv bootimg 'tftp ${loadaddr} ${serverip}:${imgname}; dcache flush; dcache off; go ${loadaddr}'
```

## SD Card

Files:

```bash
/card
├── bootcode.bin
├── config.txt
├── fixup.dat
├── start.elf
├── u-boot.bin
└── uboot.env
```

Contents of `config.txt`:

```bash
enable_uart=1
arm_64bit=1
dtoverlay=pi3-disable-bt
kernel=u-boot.bin
```

## Useful Links

- https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
- https://github.com/hermanhermitage/videocoreiv
- https://github.com/rust-embedded/rust-raspi3-tutorial
- https://github.com/bztsrc/raspi3-tutorial
- https://github.com/raspberrypi/firmware/wiki/Mailbox-framebuffer-interface
- https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#set-clock-rate
- https://github.com/brianwiddas/pi-baremetal/blob/master/framebuffer.c
- https://github.com/BrianSidebotham/arm-tutorial-rpi/tree/master/part-5
- https://docs.sel4.systems/Hardware/Rpi3.html
- https://a-delacruz.github.io/ubuntu/rpi3-setup-64bit-uboot.html
- https://github.com/hermanhermitage/videocoreiv/wiki/VideoCore-IV-Programmers-Manual
