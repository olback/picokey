# Picokey

```bash
git clone git@github.com/olback/picokey && cd picokey
head -c 44 < /dev/urandom > keyiv
mkdir ~/.config/picokey
```

`~/.config/picokey/Config.toml`:
```toml
# Command to unlock (e.g. kill screen saver)
unlock-command = ""

# 8 bytes per id in base64
pico-ids = [
    "AAAAAAAAAAA" # Example id
]
```

