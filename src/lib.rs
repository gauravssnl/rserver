//! # RServer
//!
//! A library/app for intercepting/sniffing TCP requests, modifying headers and responses.

use futures::future::poll_fn;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, Interest},
    net::{TcpListener, TcpStream},
};

pub mod config;
pub mod request;

use config::Config;

use request::Request;

/// Start server at the given host and port which will handle requests recieved from clients/apps.
///
/// Example
/// ```no_run
/// use rserver::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::default();
///     rserver::start_server(&config).await
/// }
/// ```
pub async fn start_server(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(address.clone()).await?;
    println!("Serving on {}", address);
    loop {
        let mut client = listener.accept().await?.0;
        tokio::task::spawn(async move { handle_client(&mut client).await });
    }
}

async fn handle_client(
    client: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_buffer = read_stream(client).await?;
    println!(
        "******************* Request Received *****************\n{}",
        String::from_utf8_lossy(&request_buffer).trim()
    );
    let request = Request::from(request_buffer);
    // println!("request: {:?}", request);
    read_and_write_data_from_remote_server(client, request).await?;
    Ok(())
}

async fn read_stream(
    stream: &mut TcpStream,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer: Vec<u8> = Vec::new();
    let ready = stream.ready(Interest::READABLE).await?;
    // println!("Stream: {:?}", stream);
    // println!("Ready to read: {:?}", ready);
    if ready.is_readable() {
        let buffer_size: usize = 1024;
        loop {
            let mut fixed_buffer = vec![0; buffer_size];
            match stream.read(&mut fixed_buffer).await {
                Ok(n) if n == 0 => break,
                Ok(n) if n < buffer_size => {
                    buffer.append(&mut fixed_buffer[..n].to_vec());
                    break;
                }
                Ok(_) => {
                    buffer.append(&mut fixed_buffer);
                }
                Err(e) => {
                    println!("Error in reading stram data: {}", e);
                    break;
                }
            }
        }
    }
    Ok(buffer)
}

async fn read_and_write_data_from_remote_server(
    client: &mut TcpStream,
    request: Request,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Handling client request....");
    let address = format!("{}:{}", request.host, request.port);
    println!("Connecting to the remote host ({})", address);
    let mut remote = TcpStream::connect(address.clone()).await?;
    println!("Connected to the remote host ({})", address);
    if request.method == "CONNECT" {
        println!("Handling HTTPS request....");
        handle_connect(client, request, &mut remote).await?;
    } else {
        println!("Handling non-HTTPS request....");
        match remote.write(&request.raw_data).await {
            Ok(n) => println!(
                "Wrote {} bytes and data to remote: {:?}",
                n,
                String::from_utf8_lossy(&request.raw_data)
            ),
            Err(e) => println!("Write error in remote: {}", e),
        }
        match read_stream(&mut remote).await {
            Ok(response) => {
                println!(
                    "Read {} bytes and data from server: {:?}",
                    response.len(),
                    String::from_utf8_lossy(&response)
                );
                match client.write(&response).await {
                    Ok(n) => println!(
                        "Wrote {} bytes and data to client: {:?}",
                        n,
                        String::from_utf8_lossy(&response)
                    ),
                    Err(e) => println!("Write error in client: {}", e),
                }
            }
            Err(e) => println!("Write error in client: {}", e),
        }
    }

    // remote.flush().await?;
    // client.flush().await?;
    // remote.shutdown().await?;
    // client.shutdown().await?;
    println!("******** Complete Response sent to the client ********\n");

    Ok(())
}

async fn handle_connect(
    client: &mut TcpStream,
    request: Request,
    remote: &mut TcpStream,
) -> Result<((), ()), std::io::Error> {
    let empty_response = format!("{} 200 Connection Established\r\n\r\n", request.version);
    println!(
        "********** Sending Response to client **********\n{}",
        empty_response.trim()
    );
    client.write_all(empty_response.as_bytes()).await?;

    // match io::copy_bidirectional(&mut client, &mut remote).await {
    //     Ok((from_client, to_client)) => {
    //         println!(
    //             "Client wrote {} bytes and received {} bytes",
    //             from_client, to_client
    //         );
    //     }
    //     Err(e) => println!("Error in copying bi-directionally: {}", e),
    // }

    let (mut cr, mut cw) = client.split();
    let (mut rr, mut rw) = remote.split();

    let client_to_remote = async {
        let mut buffer = vec![0; 8096];
        let mut read_half = tokio::io::ReadBuf::new(&mut buffer);
        let _peeked_data_len = poll_fn(|cx| cr.poll_peek(cx, &mut read_half)).await?;
        // println!(
        //     "Peeked client data: {:?}",
        //     String::from_utf8_lossy(&buffer[..peeked_data_len])
        // );
        io::copy(&mut cr, &mut rw).await?;
        rw.shutdown().await
    };

    let remote_to_client = async {
        let mut buffer = vec![0; 8096];
        let mut read_half = tokio::io::ReadBuf::new(&mut buffer);
        let _peeked_data_len = poll_fn(|cx| rr.poll_peek(cx, &mut read_half)).await?;
        // println!(
        //     "\nPeeked remote server data: {:?}",
        //     String::from_utf8_lossy(&buffer[..peeked_data_len])
        // );
        io::copy(&mut rr, &mut cw).await?;
        cw.shutdown().await
    };

    let res = tokio::try_join!(client_to_remote, remote_to_client);
    res
    // remote.flush().await?;
    // client.flush().await?;
    // println!("Response sent for CONNECT");
}
