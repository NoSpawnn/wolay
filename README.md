# wolay - A tiny Wake-On-Lan Relay server

- I wanted to be able to Wake-On-Lan my stuff over tailscale, so run this on a small SBC

## Features

- [ ] Wake-On-LAN relay (basically done)
- [ ] Shutdown/WOL on a schedule

## Usage

### Development

1. Clone repo

```sh
git clone https://github.com/NoSpawnn/wolay.git
cd wolay
```

2. Build

  - For your current system

    ```sh
    cargo build
    ```

  - Cross compile

    ```sh
    nix build
    ```

## Refs

- [Wake-on-LAN - Wikipedia](https://en.wikipedia.org/wiki/Wake-on-LAN)
- [wake_on_lan - Rust](https://docs.rs/wake-on-lan/latest/wake_on_lan/)
