use std::net::ToSocketAddrs;

use crate::magic_packet::{MacAddress, MagicPacket};
use tiny_http::Response;

mod magic_packet;

fn main() -> std::io::Result<()> {
    serve("127.0.0.1:9090")
}

fn serve<A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {
    let addrs = addr.to_socket_addrs()?;
    let addr = addrs.into_iter().next().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "no socket addresses")
    })?;

    let server = tiny_http::Server::http(addr).unwrap();
    log::info!("Started server running on {addr}");

    loop {
        let req = server.recv()?;

        log::info!("Recieved request from {:#?}", req.remote_addr());

        let path = req.url();
        if !path.starts_with("/api/wake/") {
            let response =
                Response::from_string("Not found. Use \"/api/wake/<mac>\".").with_status_code(404);
            let _ = req.respond(response);
            continue;
        }

        if let Some(mac_str) = path.split('/').last() {
            let response: Response<_>;

            match MacAddress::try_from(mac_str) {
                Ok(mac) => {
                    let mp = MagicPacket::new(&mac);
                    mp.send()?;
                    log::info!("Successfully sent magic packet to {mac_str}");
                    response = Response::from_string(format!("Sent magic packet to {mac_str}"))
                        .with_status_code(200);
                }
                Err(e) => {
                    log::error!("Failed parsing MAC address from '{mac_str}': {e:?}");
                    response =
                        Response::from_string(format!("Malformed MAC address '{mac_str}': {e:?}"))
                            .with_status_code(500);
                }
            }

            let _ = req.respond(response);
        }
    }
}
