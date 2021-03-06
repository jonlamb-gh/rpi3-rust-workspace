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
# Disables the Bluetooth device and restores UART0/ttyAMA0 to GPIOs 14 and 15
dtoverlay=pi3-disable-bt
dtparam=i2c_arm=on
kernel=u-boot.bin
```

## Useful Links

- [Revised BCM2837 doc](https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf)
- [Bare metal boot code for ARMv8-A](http://infocenter.arm.com/help/topic/com.arm.doc.dai0527a/DAI0527A_baremetal_boot_code_for_ARMv8_A_processors.pdf)
- [Bare-metal C++ code](https://github.com/rsta2/circle)
- [Tools and info on the  Broadcom VideoCore IV](https://github.com/hermanhermitage/videocoreiv)
- [Bare metal RPi3 in C](https://github.com/bztsrc/raspi3-tutorial)
- [Wiki: Mailbox framebuffer interface](https://github.com/raspberrypi/firmware/wiki/Mailbox-framebuffer-interface)
- [Wiki: Mailbox property interface](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface)
- [Raspberry Pi bare metal experiments](https://github.com/brianwiddas/pi-baremetal/blob/master/framebuffer.c)
- [Raspberry-Pi Bare Metal Tutorial](https://github.com/BrianSidebotham/arm-tutorial-rpi)
- [Raspberry Pi 3 with 64-bit U-Boot](https://a-delacruz.github.io/ubuntu/rpi3-setup-64bit-uboot.html)
- [VideoCore-IV-Programmers-Manual](https://github.com/hermanhermitage/videocoreiv/wiki/VideoCore-IV-Programmers-Manual)
