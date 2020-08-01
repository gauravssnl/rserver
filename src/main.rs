mod lib;

fn main() {
    let (server_host, server_port) = ("127.0.0.1", 80);
    lib::start_server(server_host, server_port);
}
