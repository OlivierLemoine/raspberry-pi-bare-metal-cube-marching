set remotetimeout 10

file target/aarch64-unknown-none/debug/kernel

target extended-remote | \
    $HOME/.local/xPacks/@xpack-dev-tools/openocd/0.10.0-15.1/.content/bin/openocd \
    -c "gdb_port pipe; log_output openocd.log" -s ./tools/ -f adafruit-ft232h.cfg -f raspi4.cfg -f load_image.cfg
