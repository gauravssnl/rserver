# RServer


A asynchronous library/app for intercepting/sniffing TCP requests, modifying headers and responses.

# Install
To Install RServer, Please use the following command :

```shell
cargo install rserver
```

# Example

How to use the RServer to intercept/sniff TCP requests. RServer currently runs on 8081 port by default.

To run RServer, use the folowing command :

```shell
rserver
```

Please set the browser/system proxy as host 127.0.0.1 and port 8081 (default port of RServer) to use Rserver for intercepting all requests.

If you directly want to test RServer installation without doing the above step, please run the below command :
```shell
https_proxy=127.0.0.1:8081 curl https://www.google.com
```

####  To use rsever  Rust library , please see the below example :

```rust
use rserver::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    rserver::start_server(&config).await
}

```

# Changelog
- Asynchronous server.
- Add HTTPS / CONNECT method support.
- Minor bug fixes & other improvements.
- Modular code

# Internal API

How to read the stream data and return stream data & its length.

```rust
/// Read the stream data and return stream data & its length
fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {
    let buffer_size = 512;
    let mut request_buffer = vec![];
    // let us loop & try to read the whole request data
    let mut request_len = 0usize;
    loop {
        let mut buffer = vec![0; buffer_size];
        match stream.read(&mut buffer) {
            Ok(n) => {
               
                if n == 0 {
                    break;
                } else {
                    request_len += n;

                    // we need not read more data in case we have read less data than buffer size
                    if n < buffer_size {
                        // let us only append the data how much we have read rather than complete existing buffer data
                        // as n is less than buffer size
                        request_buffer.append(&mut buffer[..n].to_vec()); // convert slice into vec
                        break;
                    } else {
                        // append complete buffer vec data into request_buffer vec as n == buffer_size
                        request_buffer.append(&mut buffer);
                    }
                }
            }
            Err(e) => {
                println!("Error in reading stream data: {:?}", e);
                break;
            }
        }
    }

    (request_buffer, request_len)
}

```
# 


To-Do
- [] Add command line flags for server config
- [ ] Modifying/replacing Request Headers
- [ ] Modifying/replacing Reponse Headers 

