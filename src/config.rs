pub struct Config {
    pub host: String,
    pub port: i32,
    pub enable_proxy: bool,
    pub proxy_host: String,
    pub proxy_port: i32,
}

impl Config {
    fn new(
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
            port: 8081,
            enable_proxy: false,
            proxy_host: String::default(),
            proxy_port: i32::default(),
        }
    }
}
