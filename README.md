# Picokey

```bash
git clone git@github.com/olback/picokey && cd picokey
head -c 44 < /dev/urandom > keyiv
cd pico
mkdir build && cd build
cmake ..
cd ..
tools/build.sh
# Boot into bootloader now ... continue
tools/flash.sh
cd ..
mkdir ~/.config/picokey
mkdir -p ~/.config/systemd/user/
cp picokey.service ~/.config/systemd/user
cd host
cargo install --path .
```

Edit `~/.config/picokey/Config.toml`:
```toml
# Command to unlock (e.g. kill screen saver)
unlock-command = "pkill swaylock"

# 8 bytes per id in base64
pico-ids = [
    "AAAAAAAAAAA" # Example id
]
```

Start service:
```bash
systemctl enable --user picokey
systemctl start --user picokey
```

