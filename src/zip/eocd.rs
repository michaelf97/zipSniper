use std::convert::TryInto;

pub const SIGNATURE_32: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];
pub const SIGNATURE_64: [u8; 4] = [0x50, 0x4b, 0x06, 0x06];

pub enum WordSize {
    Bit32,
    Bit64,
}

pub struct Eocd {
    bytes: Box<[u8]>,
    signature: [u8; 4],
    //disk_number: u16,
    //disk_where_cd_starts: u32,
    //number_of_cd_records_on_disk: u32,
    //number_of_cd_records: u32,
    //size_of_cd: u64,
    //offset_of_cd_from_start: u64,
    //comment: Option<String>,
}

impl Eocd {
    pub fn from(bytes: &[u8]) -> Eocd {
        Eocd {
            bytes: Box::from(bytes),
            signature: [bytes[0], bytes[1], bytes[2], bytes[3]],
        }
    }

    pub fn verify_wordsize(&self) -> WordSize {
        println!("{:?}", self.bytes);
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
        }).expect("Invalid UTF-8")
    }
}
