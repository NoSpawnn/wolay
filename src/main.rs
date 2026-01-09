use std::net::{IpAddr, ToSocketAddrs};

use crate::magic_packet::{MacAddress, MagicPacket};
use clap::Parser;
use tiny_http::Response;

mod magic_packet;

static DEFAULT_ADDR: &str = "127.0.0.1";
static DEFAULT_PORT: u16 = 6789;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short = 'a', long, default_value = DEFAULT_ADDR)]
    listen_addr: IpAddr,

    #[arg(short = 'p', long, default_value_t = DEFAULT_PORT)]
    listen_port: u16,
}

fn main() -> std::io::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    serve((args.listen_addr, args.listen_port))
}

fn serve<A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {
    let addrs = addr.to_socket_addrs()?;
    let addr = addrs.into_iter().next().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "no socket addresses")
    })?;

    let server = tiny_http::Server::http(addr).unwrap();
    log::info!("Started wolay server listening on {addr}");

    loop {
        let req = server.recv()?;

        log::debug!(
            "Recieved request from {:?} for {}",
            req.remote_addr(),
            req.url()
        );

        let response: Response<_>;
        let path = req.url();
        if !path.starts_with("/api/wake/") {
            response =
                Response::from_string("Not found. Use \"/api/wake/<mac>\".").with_status_code(404);
            if let Err(e) = req.respond(response) {
                log::error!("Failed responding: {e}");
            }
            continue;
        }

        if let Some(mac_str) = path.split('/').last() {
            match MacAddress::try_from(mac_str) {
                Ok(mac) => {
                    let mp = MagicPacket::new(&mac);
                    if let Err(e) = mp.send() {
                        log::error!("Failed sending magic packet to {mac_str}: {e}");
                        response = Response::from_string(format!(
                            "Failed sending magic packet to {mac_str}"
                        ))
                        .with_status_code(500);
                    } else {
                        log::info!("Sent magic packet to {mac_str}");
                        response = Response::from_string(format!("Sent magic packet to {mac_str}"))
                            .with_status_code(200);
                    }
                }
                Err(e) => {
                    log::error!("Failed parsing MAC address from '{mac_str}': {e:?}");
                    response =
                        Response::from_string(format!("Malformed MAC address '{mac_str}': {e:?}"))
                            .with_status_code(500);
                }
            }

            if let Err(e) = req.respond(response) {
                log::error!("Failed responding: {e}");
            }
        }
    }
}
