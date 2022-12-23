use futures::future::poll_fn;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, Interest},
    net::{TcpListener, TcpStream},
};

use crate::{config::Config, Request};

/// RServer's Server struct.
pub struct Server {
    pub config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// Start server at the given host and port which will handle requests recieved from clients/apps.
    ///
    /// Example
    /// ```no_run
    /// use rserver::Config;
    /// use rserver::Server;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = Config::default();
    ///     let server = Server::new(config);
    ///     server.start().await
    /// }
    /// ```
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(address.clone()).await?;
        println!("Serving on {}", address);
        loop {
            let mut client = listener.accept().await?.0;
            let config = self.config.clone();
            tokio::task::spawn(async move { handle_client(&mut client, config).await });
        }
    }
}

// Handle client TcpStream.
async fn handle_client(
    client: &mut TcpStream,
    config: Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_buffer = read_stream(client).await?;
    println!(
        "******************* Request Received *****************\n{}\n",
        String::from_utf8_lossy(&request_buffer).trim()
    );
    let request = Request::from(request_buffer);
    // println!("request: {:?}", request);
    connect_and_handle_client_request(client, request, &config).await?;
    Ok(())
}

/// Read data from a TcpStream. Data is returned as Vec<u8> (bytes).
pub async fn read_stream(
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

/// Connect to remote address or proxy & handle client request.
async fn connect_and_handle_client_request(
    client: &mut TcpStream,
    request: Request,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Handling client request....");
    let address = if config.enable_proxy {
        format!("{}:{}", config.proxy_host, config.proxy_port)
    } else {
        format!("{}:{}", request.host, request.port)
    };
    println!("Connecting to the remote host ({})", address);
    let mut remote = TcpStream::connect(address.clone()).await?;
    println!("Connected to the remote host ({})", address);
    match request.method.as_str() {
        "CONNECT" => handle_connect(client, request, &mut remote).await?,
        _ => handle_default(client, request, &mut remote).await?,
    }

    // remote.flush().await?;
    // client.flush().await?;
    // remote.shutdown().await?;
    // client.shutdown().await?;
    println!("******** Complete Response sent to the client ********\n");

    Ok(())
}

/// Handle requests which are not CONNECT, i.e. GET, POST, etc.
async fn handle_default(
    client: &mut TcpStream,
    request: Request,
    remote: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Handling non-HTTPS request....");
    match remote.write(&request.raw_data).await {
        Ok(n) => println!(
            "Wrote {} bytes and data to remote: {:?}",
            n,
            String::from_utf8_lossy(&request.raw_data)
        ),
        Err(e) => println!("Write error in remote: {}", e),
    }
    match read_stream(remote).await {
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
    Ok(())
}

/// Handles CONNECT method request between client & remote TcpStream.
/// Send 200 Connection Established Response first and then copy the data between
/// those strams bi-directionally.
async fn handle_connect(
    client: &mut TcpStream,
    request: Request,
    remote: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Handling HTTPS request....");
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

    tokio::try_join!(client_to_remote, remote_to_client)?;
    // remote.flush().await?;
    // client.flush().await?;
    // println!("Response sent for CONNECT");
    Ok(())
}
