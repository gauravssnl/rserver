//! # RServer
//!
//! A library for intercepting/sniffing TCP requests, modify headers and responses.

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Start server at the given host and port which will handle requests recieved from clients/apps
///
/// Example
///
/// ```no_run
///
/// fn main() {
///     let (server_host, server_port) = ("127.0.0.1", 80);
///     rserver::start_server(server_host, server_port);
/// }
/// ```

pub fn start_server(server_host: &str, server_port: u32) {
    let server_address = format!("{}:{}", server_host, server_port);
    let listener = TcpListener::bind(server_address).unwrap();
    println!("Server is running at {}:{}", server_host, server_port);
    // let us serve forever
    loop {
        let (stream, socket_address) = listener.accept().unwrap();
        println!("Got new client whose address is {}", socket_address);
        println!("stream: {:?}", stream);
        thread::spawn(|| handle_client(stream));
    }
}

/// Handle client stream and send the response
fn handle_client(mut stream: TcpStream) {
    let (request_buff, request_len) = read_request(&mut stream);
    let response = prepare_response(&request_buff, request_len);
    write_response(stream, &request_buff, response);
}

/// Read the request data and return request data & request length
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

/// Preapre the response string that has to be sent to the clients
fn prepare_response(request_buffer: &[u8], request_length: usize) -> String {
    println!("Request length: {}", request_length);
    println!("{} Request {}", "*".repeat(20), "*".repeat(20));
    let request_data = String::from_utf8_lossy(&request_buffer[..]);
    println!("{}", request_data);
    println!("{} Response for client {}", "*".repeat(20), "*".repeat(20));
    let (status_line, contents) = (
        "HTTP/1.1 200 OK\r\n\r\n",
        format!(
            "\
    <html>
    <head><title>RServer</title></head>
    <body><div>Hello, world</div><div>Request Header: <br>{}</body></html>\r\n\r\n",
            // the buffer might have less data than its size as of now
            String::from_utf8_lossy(&request_buffer[..request_length])
        ),
    );
    let response = format!("{}{}", status_line, contents);
    println!("{}", response);
    response
}

/// Write response to client stream
fn write_response(mut stream: TcpStream, request_buffer: &[u8], response: String) {
    // let us simulate Delay to test multi-threading
    if request_buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(std::time::Duration::from_secs(5));
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response sent to the client successfully");
    println!("{}", "*".repeat(50));
}
