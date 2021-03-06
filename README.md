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


### POC 1

Start server and client in the same machine. Config client to be at the left side
of the server. When moving the mouse to the left of the server screen (0,y), the mouse
goes to the right of the client screen (max_x-1, y) (both screens are the same in this case).
Coming back with the mouse to the right of the client screen makes the mouse go back to the
left side of the server (1, y).

### Proposed implementation

When the server starts, it starts a thread to monitor the mouse position. This thread constantly
updates a Mouse struct with the new position, and checks if the mouse has reached a side
that has a client.

