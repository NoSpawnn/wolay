use std::net::{Ipv4Addr, UdpSocket};

// magic packet is a frame that contains 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal),
// followed by sixteen repetitions of the target computer's 48-bit MAC address,
// for a total of 102 bytes
#[derive(Debug)]
pub struct MagicPacket {
    bytes: [u8; Self::SIZE],
}

impl MagicPacket {
    const HEADER: [u8; 6] = [0xFF; 6];
    const SIZE: usize = 102;

    pub fn new(mac_address: &MacAddress) -> Self {
        let mut bytes = [0u8; Self::SIZE];

        bytes[..6].copy_from_slice(&Self::HEADER);
        bytes[6..]
            .chunks_exact_mut(6)
            .for_each(|c| c.copy_from_slice(&mac_address.bytes));

        Self { bytes }
    }

    /// Send this magic packet, letting the OS choose source port and interface.
    pub fn send(&self) -> std::io::Result<()> {
        let source = (Ipv4Addr::new(0, 0, 0, 0), 0);
        let dest = (Ipv4Addr::new(255, 255, 255, 255), 9);

        let socket = UdpSocket::bind(source)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.bytes, dest)?;

        Ok(())
    }

    pub fn magic_bytes(&self) -> &[u8; Self::SIZE] {
        &self.bytes
    }
}

#[derive(Debug)]
pub struct MacAddress {
    pub bytes: [u8; 6],
}

impl From<[u8; 6]> for MacAddress {
    fn from(bytes: [u8; 6]) -> Self {
        Self { bytes }
    }
}

#[derive(Debug)]
pub enum MacAddressParseError {
    InvalidLen(usize),
    ParseIntError(std::num::ParseIntError),
}

impl TryFrom<&str> for MacAddress {
    type Error = MacAddressParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hex = s.replace([':'], "");

        if hex.len() != 12 {
            return Err(Self::Error::InvalidLen(hex.len()));
        }

        let mut bytes = [0u8; 6];
        for i in 0..6 {
            let b = &hex[i * 2..i * 2 + 2];
            bytes[i] = u8::from_str_radix(b, 16).map_err(|e| Self::Error::ParseIntError(e))?;
        }

        Ok(Self { bytes })
    }
}
