use std::env;
use clap::Clap;

#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

#[derive(Clap)]
struct Opts {
    protocol: String,
    role: String,
    address: String
}

fn main() {
    let opts: Opts = Opts::parse();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let protocol = opts.protocol.as_str();
    let role = opts.role.as_str();
    let address = opts.address.as_str();
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                udp_client::communicate(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
