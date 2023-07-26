use std::fmt;

pub const SIGNATURE_32: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];
pub const SIGNATURE_64: [u8; 4] = [0x50, 0x4b, 0x06, 0x06];

pub enum WordSize {
    Bit32,
    Bit64,
}

pub struct Eocd {
    bytes: Box<[u8]>,
    signature: [u8; 4],
}

#[allow(dead_code)]
impl fmt::Display for Eocd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "### End of Central Directory record (EOCD) ###\n");
        write!(f, "Memory Offset \t Attribute Name \t Value\n");
        match self.verify_wordsize() {
            WordSize::Bit32 => {
                write!(
                    f,
                    "00000000 \t SIGNATURE \t\t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[3], self.bytes[2], self.bytes[1], self.bytes[0]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000004 \t Number of this disk \t {:02x}{:02x}\n",
                    self.bytes[5], self.bytes[4]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000006 \t Central Dir Disk No \t {:02x}{:02x}\n",
                    self.bytes[7], self.bytes[6]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000008 \t Entries in this disk \t {:02x}{:02x}\n",
                    self.bytes[9], self.bytes[8]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "0000000A \t Total entries \t\t {:02x}{:02x}\n",
                    self.bytes[11], self.bytes[10]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "0000000C \t Size of Central Dir \t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[15], self.bytes[14], self.bytes[13], self.bytes[12]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000010 \t Offset to Central Dir \t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[19], self.bytes[18], self.bytes[17], self.bytes[16]
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000014 \t Comment Length \t {:02x}{:02x}\n",
                    self.bytes[21], self.bytes[20],
                )
            }
            WordSize::Bit64 => {
                write!(
                    f,
                    "00000000 \t SIGNATURE \t\t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[3], self.bytes[2], self.bytes[1], self.bytes[0],
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "0000000C \t Version Used \t\t {:02x}{:02x}\n",
                    self.bytes[13], self.bytes[12],
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "0000000E \t Version Needed \t\t {:02x}{:02x}\n",
                    self.bytes[15], self.bytes[14],
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000010 \t Number of this disk \t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[19], self.bytes[18], self.bytes[17], self.bytes[16],
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000014 \t Central Dir Disk No \t {:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[23], self.bytes[22], self.bytes[21], self.bytes[20],
                )
                .expect("Error printing out CD");
                write!(
                    f,
                    "00000018 \t Entries in this disk \t {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}\n",
                    self.bytes[31], self.bytes[30], self.bytes[29], self.bytes[28], self.bytes[27], self.bytes[26], self.bytes[25], self.bytes[24]
                ).expect("Error printing out CD");
                write!(
                    f,
                    "00000020 \t Total Entries \t\t {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    self.bytes[39], self.bytes[38], self.bytes[37], self.bytes[36], self.bytes[35], self.bytes[34], self.bytes[33], self.bytes[32]
                ).expect("Error printing out CD");
                write!(
                    f,
                    "00000028 \t Size of Central Dir \t {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    self.bytes[47], self.bytes[46], self.bytes[45], self.bytes[44], self.bytes[43], self.bytes[42], self.bytes[41], self.bytes[40]
                ).expect("Error printing out CD");
                write!(
                    f,
                    "00000030 \t Offset to Central Dir \t {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    self.bytes[55], self.bytes[54], self.bytes[53], self.bytes[52], self.bytes[51], self.bytes[50], self.bytes[49], self.bytes[48]
                )
            }
        }
    }
}

#[allow(dead_code)]
impl Eocd {
    pub fn from(bytes: &[u8]) -> Eocd {
        Eocd {
            bytes: Box::from(bytes),
            signature: [bytes[0], bytes[1], bytes[2], bytes[3]],
        }
    }

    pub fn verify_wordsize(&self) -> WordSize {
        match self.signature {
            SIGNATURE_32 => WordSize::Bit32,
            SIGNATURE_64 => WordSize::Bit64,
            _ => panic!("Invalid Signature"),
        }
    }

    pub fn disk_number(&self) -> u32 {
        u32::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[4], self.bytes[5], 0, 0],
            WordSize::Bit64 => [
                self.bytes[16],
                self.bytes[17],
                self.bytes[18],
                self.bytes[19],
            ],
        })
    }

    pub fn disk_where_cd_starts(&self) -> u32 {
        u32::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[6], self.bytes[7], 0, 0],
            WordSize::Bit64 => [
                self.bytes[20],
                self.bytes[21],
                self.bytes[22],
                self.bytes[23],
            ],
        })
    }

    pub fn number_of_cd_records_on_disk(&self) -> u32 {
        u32::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[8], self.bytes[9], 0, 0],
            WordSize::Bit64 => [
                self.bytes[24],
                self.bytes[25],
                self.bytes[26],
                self.bytes[27],
            ],
        })
    }

    pub fn total_number_of_cd_records(&self) -> u32 {
        u32::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[10], self.bytes[11], 0, 0],
            WordSize::Bit64 => [
                self.bytes[32],
                self.bytes[33],
                self.bytes[34],
                self.bytes[35],
            ],
        })
    }

    pub fn size_of_cd(&self) -> u64 {
        u64::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [
                self.bytes[12],
                self.bytes[13],
                self.bytes[14],
                self.bytes[15],
                0,
                0,
                0,
                0,
            ],
            WordSize::Bit64 => [
                self.bytes[40],
                self.bytes[41],
                self.bytes[42],
                self.bytes[43],
                self.bytes[44],
                self.bytes[45],
                self.bytes[46],
                self.bytes[47],
            ],
        })
    }

    pub fn cd_start_offset(&self) -> u64 {
        u64::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [
                self.bytes[16],
                self.bytes[17],
                self.bytes[18],
                self.bytes[19],
                0,
                0,
                0,
                0,
            ],
            WordSize::Bit64 => [
                self.bytes[48],
                self.bytes[49],
                self.bytes[50],
                self.bytes[51],
                self.bytes[52],
                self.bytes[53],
                self.bytes[54],
                self.bytes[55],
            ],
        })
    }

    pub fn comment_length(&self) -> u16 {
        u16::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[20], self.bytes[21]],
            WordSize::Bit64 => return (self.bytes.len() - 56).try_into().unwrap(),
        })
    }

    pub fn comment(&self) -> String {
        String::from_utf8(match self.verify_wordsize() {
            WordSize::Bit32 => self.bytes[22..].to_vec(),
            WordSize::Bit64 => self.bytes[56..].to_vec(),
        })
        .expect("Invalid UTF-8")
    }

    pub fn version_made_by(&self) -> Option<u16> {
        Some(u16::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => return None,
            WordSize::Bit64 => [self.bytes[12], self.bytes[13]],
        }))
    }

    pub fn version_needed_to_extract(&self) -> Option<u16> {
        Some(u16::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => return None,
            WordSize::Bit64 => [self.bytes[14], self.bytes[15]],
        }))
    }
}
