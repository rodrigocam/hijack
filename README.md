# hijack
Share your mouse across computers through network.

simulate mouse/keyboard events - https://docs.rs/enigo/latest/enigo/
mesma coisa mais completa - https://github.com/autopilot-rs/autopilot-rs
escuta eventos - https://github.com/ostrosco/device_query

WinAPI, Cocoa, and Xlib or XCB


## How to run

```shell
# For server
cargo run -- --config config.toml

# For clients
cargo run -- --client client_name --server 192.0.0.1
```

