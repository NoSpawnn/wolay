# wolay - A tiny Wake-On-Lan Relay server

- I wanted to be able to Wake-On-Lan my stuff over tailscale, so run this on a small SBC

## Features

- [ ] Wake-On-LAN relay (basically done)
- [ ] Shutdown/WOL on a schedule

## Usage

  ```
  ./wolay --help
  Usage: wolay [OPTIONS]
  
  Options:
    -a, --listen-addrs <LISTEN_ADDRS>...  Define multiple addresses for wolay to listen on (e.g. 127.0.0.1:6789)
    -l, --listen-addr <LISTEN_ADDR>       Define IP for wolay to listen on [default: 127.0.0.1]
    -p, --listen-port <LISTEN_PORT>       Define port for wolay to listen on [default: 6789]
    -h, --help                            Print help
  ```

  - Run wolay as above
  - Make a web request to `ip:port/api/wake/<mac>`
    - e.g. with `curl`
      `curl pi@raspberrypi.internal:6789/api/wake/aaaaaaaaaaaa`

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
