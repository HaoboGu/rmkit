const UF2_MAGIC_START0: u32 = 0x0A324655; // "UF2\n"
const UF2_MAGIC_START1: u32 = 0x9E5D5157;
const UF2_MAGIC_END: u32 = 0x0AB16F30;

/// uf2 Block
pub struct Block {
    /// address inside uf2 file
    pub address: u32,
    /// data
    pub bytes: [u8; 256],
}

impl Block {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            bytes: [0; 256],
        }
    }

    /// encode this block to uf2
    pub fn encode(&self, block_no: u32, number_of_blocks: u32, family_id: Option<u32>) -> Vec<u8> {
        let flags: u32 = match family_id {
            Some(0x0) => 0x0,
            Some(_) => 0x2000,
            None => 0x0,
        };

        let mut header = vec![];
        header.extend_from_slice(&UF2_MAGIC_START0.to_le_bytes());
        header.extend_from_slice(&UF2_MAGIC_START1.to_le_bytes());
        header.extend_from_slice(&flags.to_le_bytes());
        header.extend_from_slice(&self.address.to_le_bytes());
        header.extend_from_slice(&(256u32).to_le_bytes()); // Fixed size
        header.extend_from_slice(&block_no.to_le_bytes());
        header.extend_from_slice(&number_of_blocks.to_le_bytes());
        if let Some(family_id) = family_id {
            header.extend_from_slice(&family_id.to_le_bytes());
        } else {
            header.extend_from_slice(&(0x00_u32).to_le_bytes());
        }

        // Add the block's data
        header.extend_from_slice(&self.bytes);

        // Pad with 0x00 to make it 512 bytes - 4 (for the footer)
        while header.len() < 512 - 4 {
            header.push(0x00);
        }

        header.extend_from_slice(&UF2_MAGIC_END.to_le_bytes());

        header
    }
}
