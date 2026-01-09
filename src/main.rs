mod magic_packet;

use magic_packet::MagicPacket;

fn main() -> std::io::Result<()> {
    let m: magic_packet::MacAddress = [0x5B, 0xEA, 0x4F, 0xAC, 0x05, 0xCA];
    let p = MagicPacket::new(&m);

    p.send()?;

    Ok(())
}
