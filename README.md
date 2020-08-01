# RServer


A library for intercepting/sniffing TCP requests, modify headers and responses.

# Example

How to use the RServer  to intercept/sniff TCP requests.

```rust
use rserver;

fn main() {
    let (server_host, server_port) = ("127.0.0.1", 80);
    rserver::start_server(server_host, server_port);
}

```