use crate::magic_packet::{MacAddress, MagicPacket};
use tiny_http::Response;

mod magic_packet;

fn main() -> std::io::Result<()> {
    serve()
}

fn serve() -> std::io::Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:9090").unwrap();

    loop {
        let req = server.recv()?;

        let path = req.url();
        if !path.starts_with("/api/wake/") {
            let response =
                Response::from_string("Not found. Use \"/api/wake/<mac>\".").with_status_code(404);
            let _ = req.respond(response);
            continue;
        }

        let mac_str = path.split('/').last().unwrap();
        let mac = MacAddress::try_from(mac_str).unwrap();
        let mp = MagicPacket::new(&mac);

        mp.send().unwrap();

        let response = Response::from_string(format!("Sent magic packet to {}", mac_str));
        req.respond(response).unwrap();
    }
}
