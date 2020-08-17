mod lib;
use std::env;

fn main() {
    /* 
    let us declare default local server host and sever port 
    that will be used in case not all commandline args are passed .
    */
    let (mut server_host, mut server_port) = ("127.0.0.1", 80);
    // let us add logic to get local server host and sever port from command line.
    // RServer will run at the input server host and server port.
    let args = env::args().collect::<Vec<String>>();    // args is  a Vector of String
    // let us check command line argument length , it should be 3 as 1st argument is the command used to invoke the program
    if args.len() < 3 {
        println!("Error - Not enough arguments supplied.\nPlease specify local server host and port as command-line arguments.");
        println!("Using default host '{}' and port '{}'.", server_host, server_port);
    } else {
        // First assign the server host
        server_host = &args[1];
        // Second assign the 
        server_port = match args[2].trim().parse() {
            Ok(num) => num,
            Err(_) => {     // handle error and continue the program
                println!("Error - Pass port as number.");
                println!("Using default port '{}'", server_port);
                // return default port value
                server_port
            }
        };

    }
    // println!("server_host: {}", server_host);
    // println!("server_port: {}", server_port);
    lib::start_server(server_host, server_port);
}
