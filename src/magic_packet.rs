pub type MacAddress = [u8; 6];

// magic packet is a frame that contains 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal),
// followed by sixteen repetitions of the target computer's 48-bit MAC address,
// for a total of 102 bytes
#[derive(Debug)]
pub struct MagicPacket {
    pub bytes: [u8; Self::SIZE],
}

impl MagicPacket {
    const HEADER: [u8; 6] = [0xFF; 6];
    const SIZE: usize = 102;

    pub fn new(mac_address: &MacAddress) -> Self {
        let mut bytes = [0u8; Self::SIZE];

        bytes[..6].copy_from_slice(&Self::HEADER);
        bytes[6..]
            .chunks_exact_mut(6)
            .for_each(|c| c.copy_from_slice(mac_address));

        Self { bytes }
    }
}
