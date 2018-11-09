# fb-test-project

## Dependencies

TODO

## Building

```
cargo xbuild --target=aarch64-unknown-none
```

## QEMU Simulation

```bash
qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial stdio
```
