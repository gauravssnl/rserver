use crate::CliOption;

/// RServer's Config struct.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub enable_proxy: bool,
    pub proxy_host: String,
    pub proxy_port: i32,
}

impl Config {
    pub fn new(
        host: String,
        port: i32,
        enable_proxy: bool,
        proxy_host: String,
        proxy_port: i32,
    ) -> Self {
        Config {
            host,
            port,
            enable_proxy,
            proxy_host,
            proxy_port,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8080,
            enable_proxy: false,
            proxy_host: String::default(),
            proxy_port: i32::default(),
        }
    }
}

impl From<CliOption> for Config {
    fn from(cli_option: CliOption) -> Self {
        Self {
            host: cli_option.host,
            port: cli_option.port,
            enable_proxy: matches!(cli_option.enable_proxy.as_str(), "true"),
            proxy_host: match cli_option.proxy_host {
                Some(p) => p,
                _ => String::default(),
            },
            proxy_port: match cli_option.proxy_port {
                Some(p) => p,
                _ => i32::default(),
            },
        }
    }
}
