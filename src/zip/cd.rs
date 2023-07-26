use std::fmt;

pub const CD_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];
const EXTRA_FIELD_SIGNATURE: [u8; 2] = [0x01, 0x00];

#[derive(Debug)]
pub enum WordSize {
    Bit32,
    Bit64,
}

#[derive(Debug)]
pub struct Cd {
    bytes: Box<[u8]>,
    signature: [u8; 4],
}

pub struct ExtraField<'a> {
    bytes: Box<&'a [u8]>,
    signature: [u8; 2],
}

#[allow(dead_code)]
impl fmt::Display for Cd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "### Central Directory File: {} ###\n", self.file_name());
        write!(f, "Memory Offset \t Attribute Name \t Value\n");
        match self.verify_wordsize() {
            WordSize::Bit32 => {
                write!(
                    f,
                    "00000004 \t Version Used \t\t {:02X}{:02X}\n",
                    self.bytes[5], self.bytes[4],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000006 \t Min Version Needed \t {:02X}{:02X}\n",
                    self.bytes[7], self.bytes[6],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000008 \t General Purpose Flag \t {:02X}{:02X}\n",
                    self.bytes[9], self.bytes[8],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000000A \t Compression Method \t {:02X}{:02X}\n",
                    self.bytes[11], self.bytes[10],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000000C \t Modification Time \t {:02X}{:02X}\n",
                    self.bytes[13], self.bytes[12],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000000E \t Modification Date \t {:02X}{:02X}\n",
                    self.bytes[15], self.bytes[14],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000000F \t CRC-32 Hash \t\t {:02X}{:02X}{:02X}{:02X}\n",
                    self.bytes[19], self.bytes[18], self.bytes[17], self.bytes[16],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000014 \t Compressed Size \t {:02X}{:02X}{:02X}{:02X}\n",
                    self.bytes[23], self.bytes[22], self.bytes[21], self.bytes[20],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000018 \t Uncompressed Size \t {:02X}{:02X}{:02X}{:02X}\n",
                    self.bytes[27], self.bytes[26], self.bytes[25], self.bytes[24],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000001C \t File Name Length \t {:02X}{:02X}\n",
                    self.bytes[29], self.bytes[28],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000001E \t Extra Field Length \t {:02X}{:02X}\n",
                    self.bytes[31], self.bytes[30],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000020 \t File Comment Length \t {:02X}{:02X}\n",
                    self.bytes[33], self.bytes[32],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000022 \t Disk Number \t\t {:02X}{:02X}\n",
                    self.bytes[35], self.bytes[34],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000024 \t Internal Attributes \t {:02X}{:02X}\n",
                    self.bytes[37], self.bytes[36],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "00000026 \t External Attributes \t {:02X}{:02X}{:02X}{:02X}\n",
                    self.bytes[41], self.bytes[40], self.bytes[39], self.bytes[38],
                )
                .expect("Error printing out EOCD");
                write!(
                    f,
                    "0000002A \t Local Header Offset \t {:02X}{:02X}{:02X}{:02X}\n",
                    self.bytes[45], self.bytes[44], self.bytes[43], self.bytes[42],
                )
                .expect("Error printing out EOCD");
                write!(f, "0000002E \t File Name \t\t '{}'\n", self.file_name())
                    .expect("Error printing out EOCD");
                write!(
                    f,
                    "{:08X} \t File Comment \t\t '{}'\n",
                    0x2a + self.file_name_length() + self.extra_field_length(),
                    self.file_comment(),
                )
            }
            WordSize::Bit64 => {
                write!(f, "Print Statement not implemented yet.")
            }
        }
    }
}

#[allow(dead_code)]
impl Cd {
    pub fn from(bytes: &[u8]) -> Cd {
        Cd {
            bytes: Box::from(bytes),
            signature: [bytes[0], bytes[1], bytes[2], bytes[3]],
        }
    }

    pub fn verify_wordsize(&self) -> WordSize {
        let _compressed_size = [
            self.bytes[20],
            self.bytes[21],
            self.bytes[22],
            self.bytes[23],
        ];
        let _uncompressed_size = [
            self.bytes[24],
            self.bytes[25],
            self.bytes[26],
            self.bytes[27],
        ];
        let _relative_offset = [
            self.bytes[42],
            self.bytes[43],
            self.bytes[44],
            self.bytes[45],
        ];
        let _disk_number = [self.bytes[34], self.bytes[35]];

        if _compressed_size == [0xff, 0xff, 0xff, 0xff]
            || _uncompressed_size == [0xff, 0xff, 0xff, 0xff]
            || _relative_offset == [0xff, 0xff, 0xff, 0xff]
            || _disk_number == [0xff, 0xff]
        {
            WordSize::Bit64
        } else {
            WordSize::Bit32
        }
    }

    pub fn version_made_by(&self) -> u16 {
        u16::from_le_bytes([self.bytes[4], self.bytes[5]])
    }

    pub fn version_needed_to_extract(&self) -> u16 {
        u16::from_le_bytes([self.bytes[6], self.bytes[7]])
    }

    pub fn general_purpose_flag(&self) -> u16 {
        u16::from_le_bytes([self.bytes[8], self.bytes[9]])
    }

    pub fn compression_method(&self) -> u16 {
        u16::from_le_bytes([self.bytes[10], self.bytes[11]])
    }

    pub fn last_modification_time(&self) -> u16 {
        u16::from_le_bytes([self.bytes[12], self.bytes[13]])
    }

    pub fn last_modification_date(&self) -> u16 {
        u16::from_le_bytes([self.bytes[13], self.bytes[14]])
    }

    pub fn crc32_hash(&self) -> u32 {
        u32::from_le_bytes([
            self.bytes[15],
            self.bytes[16],
            self.bytes[17],
            self.bytes[18],
        ])
    }

    pub fn compressed_size(&self) -> u64 {
        u64::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [
                self.bytes[20],
                self.bytes[21],
                self.bytes[22],
                self.bytes[23],
                0,
                0,
                0,
                0,
            ],
            WordSize::Bit64 => {
                let offset = 46 + self.file_name_length() as usize;
                [
                    self.bytes[12 + offset],
                    self.bytes[13 + offset],
                    self.bytes[14 + offset],
                    self.bytes[15 + offset],
                    self.bytes[16 + offset],
                    self.bytes[17 + offset],
                    self.bytes[18 + offset],
                    self.bytes[19 + offset],
                ]
            }
        })
    }

    pub fn uncompressed_size(&self) -> u64 {
        u64::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [
                self.bytes[24],
                self.bytes[25],
                self.bytes[26],
                self.bytes[27],
                0,
                0,
                0,
                0,
            ],
            WordSize::Bit64 => {
                let offset = 46 + self.file_name_length() as usize;
                [
                    self.bytes[4 + offset],
                    self.bytes[5 + offset],
                    self.bytes[6 + offset],
                    self.bytes[7 + offset],
                    self.bytes[8 + offset],
                    self.bytes[9 + offset],
                    self.bytes[10 + offset],
                    self.bytes[11 + offset],
                ]
            }
        })
    }

    pub fn file_name_length(&self) -> u16 {
        u16::from_le_bytes([self.bytes[28], self.bytes[29]])
    }

    pub fn extra_field_length(&self) -> u16 {
        u16::from_le_bytes([self.bytes[30], self.bytes[31]])
    }

    pub fn file_comment_length(&self) -> u16 {
        u16::from_le_bytes([self.bytes[32], self.bytes[33]])
    }

    pub fn disk_number_where_file_starts(&self) -> u32 {
        u32::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [self.bytes[34], self.bytes[35], 0, 0],
            WordSize::Bit64 => {
                let offset = 46 + self.file_name_length() as usize;
                [
                    self.bytes[28 + offset],
                    self.bytes[29 + offset],
                    self.bytes[30 + offset],
                    self.bytes[31 + offset],
                ]
            }
        })
    }

    pub fn internal_file_attributes(&self) -> u16 {
        u16::from_le_bytes([self.bytes[36], self.bytes[37]])
    }

    pub fn external_file_attributes(&self) -> u32 {
        u32::from_le_bytes([
            self.bytes[38],
            self.bytes[39],
            self.bytes[40],
            self.bytes[41],
        ])
    }

    pub fn relative_offset_local_header(&self) -> u64 {
        u64::from_le_bytes(match self.verify_wordsize() {
            WordSize::Bit32 => [
                self.bytes[42],
                self.bytes[43],
                self.bytes[44],
                self.bytes[45],
                0,
                0,
                0,
                0,
            ],
            WordSize::Bit64 => {
                let offset = 46 + self.file_name_length() as usize;
                [
                    self.bytes[20 + offset],
                    self.bytes[21 + offset],
                    self.bytes[22 + offset],
                    self.bytes[23 + offset],
                    self.bytes[24 + offset],
                    self.bytes[25 + offset],
                    self.bytes[26 + offset],
                    self.bytes[27 + offset],
                ]
            }
        })
    }

    pub fn file_name(&self) -> String {
        let last_byte = 46 + self.file_name_length() as usize;
        String::from_utf8(self.bytes[46..last_byte].to_vec()).expect("Invalid UTF-8")
    }

    pub fn extra_field(&self) -> ExtraField {
        let first_byte = 46 + self.file_name_length() as usize;
        let last_byte = first_byte + self.extra_field_length() as usize;
        let signature = [self.bytes[first_byte], self.bytes[first_byte + 1]];
        if signature != EXTRA_FIELD_SIGNATURE {
            panic!("Invalid Extra Field Signature");
        }
        ExtraField {
            bytes: Box::new(&self.bytes[first_byte..last_byte]),
            signature: signature,
        }
    }

    pub fn file_comment(&self) -> String {
        let mut first_byte = (46 + self.file_name_length()) as usize;
        let mut last_byte = (46 + self.file_name_length() + self.file_comment_length()) as usize;
        match self.verify_wordsize() {
            WordSize::Bit32 => {}
            WordSize::Bit64 => {
                first_byte += self.extra_field_length() as usize;
                last_byte += self.extra_field_length() as usize;
            }
        }
        String::from_utf8(self.bytes[first_byte..last_byte].to_vec()).expect("Invalid UTF-8")
    }
}
