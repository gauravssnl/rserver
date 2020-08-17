# RServer


A library/app for intercepting/sniffing TCP requests, modifying headers and responses.

# Install
To Install RServer, Please use the following command :

```shell
cargo install rserver
```

# Example

How to use the RServer  to intercept/sniff TCP requests.

Let us consider that we want to run RServer at localhost address "127.0.0.1" and port 80. To run RServer, use the folowing command :

```shell
rserver 127.0.0.1 80
```

Please set the browser/system proxy as host 127.0.0.1 and port 80 to use Rserver for intercepting all requests.

If you directly want to test RServer installation without doing the above step, please open this URL in Web Browser :
http://127.0.0.1/ and you should see HTTP Headers sent by the Browser in that page.

####  To use rsever  Rust library , please see the below example :

```rust
use rserver;

fn main() {
    let (server_host, server_port) = ("127.0.0.1", 80);
    rserver::start_server(server_host, server_port);
}

```
# ScreenShot

![ScreenShot]( https://github.com/gauravssnl/rserver/blob/master/media/images/rserver_initial.PNG )

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

Note : Currently : Connection to online hosts are working fine with GET requests only. CONNECT requests respose reading has some issue that needs to be fixed.

To-Do

- [ ] Fix CONNECT Method Response Reading
- [ ] Modifying/replacing Request Headers
- [ ] Modifying/replacing Reponse Headers 

