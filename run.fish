cargo build && aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/debug/kernel sd_card/kernel8-rpi4.img && aarch64-none-elf-gdb -x tools/run.gdb -tui -quiet
