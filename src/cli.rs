use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rserver",
    about = "Asynchronous TCP server for intercepting requests, modifying request headers, and replacing responses"
)]
pub struct CliOption {
    /// Server host
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,

    /// Server port
    #[structopt(short, long, default_value = "8080")]
    pub port: i32,

    /// Enable proxy flag
    #[structopt(long, default_value = "false")]
    pub enable_proxy: String,

    /// Proxy host
    #[structopt(long, required_if("enable-proxy", "true"))]
    pub proxy_host: Option<String>,

    /// Proxy port
    #[structopt(long, required_if("enable-proxy", "true"))]
    pub proxy_port: Option<i32>,
}

impl CliOption {
    pub fn parse() -> Self {
        Self::from_args()
    }
}