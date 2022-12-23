//! # RServer
//!
//! A library/app for intercepting/sniffing TCP requests, modifying headers and responses.

mod cli;
mod config;
mod request;
mod server;

pub use cli::CliOption;
pub use config::Config;
pub use request::Request;
pub use server::read_stream;
pub use server::Server;
