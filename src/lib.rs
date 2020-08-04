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
        println!("Stream: {:?}", stream);
        thread::spawn(|| handle_client(stream));
    }
}

/// Handle client stream and send the response
fn handle_client(mut stream: TcpStream) {
    let (mut request_buff, _request_len) = read_stream(&mut stream);
    // Added this line for verification of reading requests correctly
    // println!(
    //     "request_buff lengeth: {}, request_len: {}",
    //     request_buff.len(),
    //     request_len
    // );
    let (method, host, host_port, _path, version) = parse_request(&request_buff);
    let (method_temp, version_temp, host_temp) = (method.clone(), version.clone(), host.clone());

    let (response_vec, _response_len) =
        send_request_and_get_response(method, host, host_port, &mut request_buff[..]);
    // println!("response_vec: {:?}", response_vec);
    // let us write the response to the local client
    write_response(stream, method_temp, version_temp, response_vec, host_temp);
    // let response = prepare_response(&request_buff);
    // write_response(stream, &request_buff, response);
}

/// Read the request data and return request data & request length
fn read_stream(stream: &mut TcpStream) -> (Vec<u8>, usize) {
    let buffer_size = 512;
    let mut request_buffer = vec![];
    // let us loop & try to read the whole request data
    let mut request_len = 0usize;
    loop {
        let mut buffer = vec![0; buffer_size];
        // println!("Reading stream data");
        match stream.read(&mut buffer) {
            Ok(n) => {
                // Added these lines for verification of reading requests correctly
                // println!("Number of bytes read from stream: {}", n);
                // println!(
                //     "Buffer data as of now: {}",
                //     String::from_utf8_lossy(&buffer[..])
                // );
                if n == 0 {
                    // Added these lines for verification of reading requests correctly
                    // println!("No bytes read");
                    break;
                } else {
                    request_len += n;

                    // we need not read more data in case we have read less data than buffer size
                    if n < buffer_size {
                        // let us only append the data how much we have read rather thann complete existing buffer data
                        // as n is less than buffer size
                        request_buffer.append(&mut buffer[..n].to_vec()); // convert slice into vec
                                                                          // Added these lines for verification of reading requests correctly
                                                                          // println!("No Need to read more data");
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
        println!("Stream read loop code ends here");
    }

    (request_buffer, request_len)
}

/// Prepare the response string that has to be sent to the clients
fn prepare_response(request_buffer: &[u8]) -> String {
    println!("{} Response for client {}", "*".repeat(20), "*".repeat(20));
    let (status_line, contents) = (
        "HTTP/1.1 200 OK\r\n\r\n",
        format!(
            "\
    <html>
    <head><title>RServer</title></head>
    <body><div>Hello, world</div><div>Request Header: <br>{}</body></html>\r\n\r\n",
            // the buffer might have less data than its size as of now
            String::from_utf8_lossy(&request_buffer)
        ),
    );
    let response = format!("{}{}", status_line, contents);
    println!("{}", response);
    response
}

/// Write response to the client stream
fn _write_response_for_localhost(mut stream: &TcpStream, request_buffer: &[u8], response: String) {
    // let us simulate Delay to test multi-threading
    if request_buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(std::time::Duration::from_secs(5));
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Response sent to the client successfully");
    println!("{}", "*".repeat(50));
}

/// Parse request buffer data to fetch Request Method, Host, Host Port, Request Path and  HTTP Version
/// return  (method, host, host_port, path, version)
fn parse_request(request_buffer: &[u8]) -> (String, String, String, String, String) {
    // println!("Request length: {}", request_buffer.len());
    println!("{} Request {}", "*".repeat(20), "*".repeat(20));
    let request_data = String::from_utf8_lossy(&request_buffer);
    println!("{}", request_data);

    let mut method = String::new();
    let mut path = String::new();
    let mut version = String::new();
    let mut host = String::new();
    let mut host_port = String::new();

    // we know that request data lines are separted by /r/n
    for (index, line) in request_data.lines().enumerate() {
        // println!("{}", line);
        // the first line contains Request Method, HTTP Version, Request Path
        if index == 0 {
            let request_first_line_data: Vec<_> = line.split(" ").collect();
            // println!("request_first_line_data:{:?}", request_first_line_data);
            if request_first_line_data.len() == 3 {
                method = request_first_line_data[0].to_string();
                path = request_first_line_data[1].to_string();
                version = request_first_line_data[2].to_string();
            } else {
                panic!("Invalid HTTP Request");
            }
        } else {
            // Header lines starts here
            // let us try reading Host value in HTTP headers
            if line.starts_with("Host:") {
                let request_line_data: Vec<_> = line.split(" ").collect();
                let host_data = request_line_data[1].to_string();

                // let us check if host port is present in header
                if host_data.contains(":") {
                    let split_host_data = host_data.split(":").collect::<Vec<_>>();
                    host = split_host_data[0].to_string();
                    host_port = split_host_data[1].to_string();
                } else {
                    host = host_data;
                    host_port = "80".to_string();
                }
            }
        }
    }
    // let request_data_lines: Vec<_> = request_data.split("\r\n").collect();
    // println!("request_data_lines: {:?}", request_data_lines);

    (method, host, host_port, path, version)
}

/// Send request to actual host
fn send_request_and_get_response(
    method: String,
    host: String,
    host_port: String,
    request_buffer: &mut [u8],
) -> (Vec<u8>, usize) {
    println!(
        "Method: {}, Host: {}. Host Port: {}",
        method, host, host_port
    );
    // if Host value is localhost, we will just send the request heders back
    if host.starts_with("127.0.0.1") || host.starts_with("localhost") {
        let response = prepare_response(&request_buffer);
        let response_vec = response.as_bytes().to_vec();
        let response_vec_len = response_vec.len();
        return (response_vec, response_vec_len);
    }
    let mut stream = TcpStream::connect(format!("{}:{}", host, host_port)).unwrap();
    println!("Sending Request to the actual server host");
    stream.write(&mut request_buffer[..]).unwrap();
    println!("Data sent to the actual host succeffully");
    println!("Trying to read response from actual server");
    let (response_vec, response_len) = read_stream(&mut stream);
    println!("Reading response from actual server complted");
    // println!("response_len: {}", response_len);
    // println!("response_vec: {:?}", response_vec);
    println!("{} Response {}", "*".repeat(20), "*".repeat(20));
    println!("{}", String::from_utf8_lossy(&response_vec));
    (response_vec, response_len)
}

/// write response to the local client stream
fn write_response(
    mut stream: TcpStream,
    method: String,
    version: String,
    response_vec: Vec<u8>,
    _host: String,
) {
    println!("Sending response to the client");
    // we need to send COONECTION ESTABLISHED message if method is CONNECT
    if method == "CONNECT" {
        println!("Sending Response for CONNECT request");
        let message = format!("{} 200 Connection Established\r\n\r\n", version);
        println!("{}", message);
        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap();
        // stream.write(&response_vec[..]).unwrap();
        // stream.flush().unwrap();
        let (mut request_buff, _request_len) = read_stream(&mut stream);
        // println!(
        //     "request_data : {:?}, _request_len: {}",
        //     request_buff, _request_len
        // );
        println!(
            "request_data : {:?}, _request_len: {}",
            String::from_utf8_lossy(&request_buff),
            _request_len
        );
        stream.write_all(&mut request_buff).unwrap();
        stream.flush().unwrap();
    } else {
        stream.write_all(&response_vec[..]).unwrap();
        stream.flush().unwrap();
    }

    println!("Response sent to the client successfully");
}
