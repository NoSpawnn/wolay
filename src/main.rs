use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    thread,
};

use crate::magic_packet::{MacAddress, MagicPacket};
use anyhow::anyhow;
use clap::Parser;
use tiny_http::Response;

mod magic_packet;

const DEFAULT_ADDR: IpAddr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
const DEFAULT_PORT: u16 = 6789;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Define multiple addresses for wolay to listen on (e.g. 127.0.0.1:6789)
    #[arg(short = 'a', long, num_args = 1..)]
    listen_addrs: Option<Vec<SocketAddr>>,

    /// Define IP for wolay to listen on
    #[arg(short = 'l', long, conflicts_with = "listen_addrs", default_value_t = DEFAULT_ADDR)]
    listen_addr: IpAddr,

    /// Define port for wolay to listen on
    #[arg(short = 'p', long, conflicts_with = "listen_addrs",default_value_t = DEFAULT_PORT)]
    listen_port: u16,
}

fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    if let Some(listen_addrs) = args.listen_addrs {
        serve(listen_addrs.into_iter())
    } else {
        serve(std::iter::once((args.listen_addr, args.listen_port)))
    }
}

fn serve<A: ToSocketAddrs>(addrs: impl Iterator<Item = A>) -> anyhow::Result<()> {
    let addrs: Vec<_> = addrs
        .map(|a| a.to_socket_addrs())
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    let mut listeners = Vec::with_capacity(addrs.len());
    for addr in addrs {
        let server =
            tiny_http::Server::http(addr).map_err(|e| anyhow!("Failed to start server: {e:?}"))?;
        let listener = thread::spawn(move || server_func(server));
        listeners.push(listener);
    }

    for listener in listeners {
        listener
            .join()
            .map_err(|e| anyhow!("Server thread panicked: {e:?}"))??;
    }

    Ok(())
}

fn server_func(server: tiny_http::Server) -> anyhow::Result<()> {
    log::info!("Started wolay server listening on {}", server.server_addr());

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
                        log::info!("Sent magic packet to {mac}");
                        response = Response::from_string(format!("Sent magic packet to {mac}"))
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
