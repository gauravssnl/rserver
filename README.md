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

How to read the request data and return request data & request length
```rust
fn read_request(stream: &mut TcpStream) -> (Vec<u8>, usize) {
    let buffer_size = 512;
    let mut request_buff = vec![];
    // let us loop & try to read the whole request data
    let mut request_len = 0usize;
    loop {
        let mut buffer = vec![0; buffer_size];
        println!("Reading stream data");
        match stream.read(&mut buffer) {
            Ok(n) => {
                println!("Top n:{}", n);
                println!("buffer data now: {}", String::from_utf8_lossy(&buffer[..]));
                if n == 0 {
                    break;
                } else {
                    request_len += n;
                    request_buff.append(&mut buffer);
                    // we need not read more data in case we have read less data than buffer size
                    if n < buffer_size {
                        println!("No Need to read more data");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error in reading stream data: {:?}", e);
                break;
            }
        }
        println!("loop stream read code ends here");
    }

    (request_buff, request_len)
}
``