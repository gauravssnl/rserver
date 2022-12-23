use rserver::CliOption;
use rserver::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_option = CliOption::parse();
    let config = cli_option.into();
    // println!("{config:?}");
    let server = Server::new(config);
    server.start().await
}
