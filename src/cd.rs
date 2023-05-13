use bytes::{Buf, Bytes};
use std::mem::size_of;

struct FieldMetaData {
    size: usize,
    offset: usize,
}

#[derive(Debug)]
pub struct Cd {
    binary: Bytes,
    pub word_size: WordSize,
}

#[derive(Debug, PartialEq)]
pub enum WordSize {
    Bit32,
    Bit64,
}

#[derive(Debug)]
pub enum CdError {
    NotValidBinary,
    InvalidUTF8ByteVector,
}

impl FieldMetaData {
    fn slice_range(&self) -> std::ops::Range<usize> {
        self.offset..(self.offset + self.size)
    }
}

impl Cd {
    /*
    This checksum signifies the start of a Central Directory entry
    */
    const CD_32_CHECKSUM: &str = "02014b50";

    pub fn from(binary: Bytes) -> Self {
        Self {
            word_size: Self::verify(&binary).unwrap(),
            binary,
        }
    }

    fn verify(binary: &Bytes) -> Result<WordSize, CdError> {
        /*
        Description: The checksum to signify the start of the EOCD section.
        Offset: 0
        Size: 4
        */
        let metadata = FieldMetaData { offset: 0, size: 4 }.slice_range();

        let first_four_bytes = binary.slice(0..size_of::<u32>());
        if first_four_bytes == Cd::cd_32_checksum() {
            return Ok(WordSize::Bit32);
        }

        Err(CdError::NotValidBinary)
    }

    pub fn version_made_by(&self) -> u16 {
        /*
        Description: Version made by
        Offset: 4
        Size: 2
        */
        let metadata = FieldMetaData { offset: 4, size: 2 }.slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn minimun_version_needed_to_extract(&self) -> u16 {
        /*
        Description: Version needed to extract (minimum)
        Offset: 6
        Size: 2
        */
        let metadata = FieldMetaData { offset: 6, size: 2 }.slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn general_purpose_bit_flag(&self) -> u16 {
        /*
        Description: General purpose bit flag
        Offset: 8
        Size: 2
        */
        let metadata = FieldMetaData { offset: 8, size: 2 }.slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn compression_method(&self) -> u16 {
        /*
        Description: Compression method
        Offset: 10
        Size: 2
        */
        let metadata = FieldMetaData {
            offset: 10,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn file_last_modification_time(&self) -> u16 {
        /*
        Description: File last modification time
        Offset: 12
        Size: 2
        */
        let metadata = FieldMetaData {
            offset: 12,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn file_last_modification_date(&self) -> u16 {
        /*
        Description: File last modification date
        Offset: 14
        Size: 2
        */
        let metadata = FieldMetaData {
            offset: 14,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn crc_32_of_uncompressed_data(&self) -> u32 {
        /*
        Description: CRC-32 of uncompressed data
        Offset: 16
        Size: 4
        */
        let metadata = FieldMetaData {
            offset: 16,
            size: 4,
        }
        .slice_range();
        self.binary.slice(metadata).get_u32_le()
    }

    pub fn compressed_size(&mut self) -> u64 {
        /*
        Description: Compressed size (or 0xffffffff for ZIP64)
        32-bit offset: 20
        32-bit size: 4
        64-bit offset:
        */
        let bit64_checksum: &[u8] = b"\xff\xff\xff\xff";
        let mut metadata = FieldMetaData { offset: 0, size: 4 }.slice_range();
        let checksum_bytes = self.binary.slice(metadata);

        metadata = if checksum_bytes.as_ref() == bit64_checksum {
            /*
            On ZIP64, the uncompressed data field is in the CD Extra Field header.
            Extra-Field offset: 12
            Extra-Field size: 8
            The Extra Field header is at the offset 46+n where n=File Name Length.
            So to extract the uncompressed data field. The equation to calculate the field we need
            46 + n + 12
            */
            self.word_size = WordSize::Bit64;
            FieldMetaData {
                offset: 58 + self.file_name_length() as usize,
                size: 8,
            }
            .slice_range()
        } else {
            self.word_size = WordSize::Bit32;
            FieldMetaData {
                offset: 20,
                size: 4,
            }
            .slice_range()
        };

        let mut bytes = self.binary.slice(metadata);
        return match self.word_size {
            WordSize::Bit32 => bytes.get_u32_le() as u64,
            WordSize::Bit64 => bytes.get_u64_le(),
        };
    }

    pub fn uncompressed_size(&mut self) -> u64 {
        /*
        Description: Unompressed size (or 0xffffffff for ZIP64)
        32-bit offset: 24
        32-bit size: 4
        */
        let bit64_checksum: &[u8] = b"\xff\xff\xff\xff";
        let mut metadata = FieldMetaData { offset: 0, size: 4 }.slice_range();
        let checksum_bytes = self.binary.slice(metadata);

        metadata = if checksum_bytes.as_ref() == bit64_checksum {
            /*
            On ZIP64, the uncompressed data field is in the CD Extra Field header.
            Extra-Field offset: 4
            Extra-Field size: 8
            The Extra Field header is at the offset 46+n where n=File Name Length.
            So to extract the uncompressed data field. The equation to calculate the field we need
            46 + n + 4
            */
            self.word_size = WordSize::Bit64;
            FieldMetaData {
                offset: 50 + self.file_name_length() as usize,
                size: 8,
            }
            .slice_range()
        } else {
            self.word_size = WordSize::Bit32;
            FieldMetaData {
                offset: 24,
                size: 4,
            }
            .slice_range()
        };

        let mut bytes = self.binary.slice(metadata);
        return match self.word_size {
            WordSize::Bit32 => bytes.get_u32_le() as u64,
            WordSize::Bit64 => bytes.get_u64_le(),
        };
    }

    pub fn file_name_length(&self) -> u16 {
        /*
        Description: File name length (n)
        32-bit offset: 28
        32-bit size: 2
        */
        let metadata = FieldMetaData {
            offset: 28,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn extra_field_length(&self) -> u16 {
        /*
        Description: Extra Field Length (m)
        32-bit offset: 30
        32-bit size: 2
        */
        let metadata = FieldMetaData {
            offset: 30,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn file_comment_length(&self) -> u16 {
        /*
        Description: File comment length (k)
        32-bit offset: 32
        32-bit size: 2
        */
        let metadata = FieldMetaData {
            offset: 32,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn disk_where_file_starts(&mut self) -> u32 {
        /*
        Description: Disk number where file starts (or 0xffff for ZIP64)
        32-bit offset: 34
        32-bit size: 2
        */

        let bit64_checksum: &[u8] = b"\xff\xff";
        let mut metadata = FieldMetaData { offset: 0, size: 4 }.slice_range();
        let checksum_bytes = self.binary.slice(metadata);

        metadata = if checksum_bytes.as_ref() == bit64_checksum {
            /*
            On ZIP64, the disk where file starts field is in the CD Extra Field header.
            Extra-Field offset: 28
            Extra-Field size: 4
            The Extra Field header is at the offset 46+n where n=File Name Length.
            So to extract the uncompressed data field. The equation to calculate the field we need
            46 + n + 28
            */
            self.word_size = WordSize::Bit64;
            FieldMetaData {
                offset: 74 + self.file_name_length() as usize,
                size: 4,
            }
            .slice_range()
        } else {
            self.word_size = WordSize::Bit32;
            FieldMetaData {
                offset: 34,
                size: 2,
            }
            .slice_range()
        };

        let mut bytes = self.binary.slice(metadata);
        return match self.word_size {
            WordSize::Bit32 => bytes.get_u16_le() as u32,
            WordSize::Bit64 => bytes.get_u32_le(),
        };
    }

    pub fn internal_file_attributes(&self) -> u16 {
        /*
        Description: Internal file attributes
        32-bit offset: 36
        32-bit size: 2
        */
        let metadata = FieldMetaData {
            offset: 36,
            size: 2,
        }
        .slice_range();
        self.binary.slice(metadata).get_u16_le()
    }

    pub fn external_file_attributes(&self) -> u32 {
        /*
        Description: External file attributes
        32-bit offset: 38
        32-bit size: 4
        */
        let metadata = FieldMetaData {
            offset: 38,
            size: 4,
        }
        .slice_range();
        self.binary.slice(metadata).get_u32_le()
    }

    pub fn file_name(&self) -> Result<String, CdError> {
        /*
        Description: File Name
        32-bit offset: 46
        32-bit size: n
        */
        let metadata = FieldMetaData {
            offset: 46,
            size: self.file_name_length() as usize,
        }
        .slice_range();
        let utf8_bytes = self.binary.slice(metadata).to_vec();
        match String::from_utf8(utf8_bytes) {
            Ok(file_name) => Ok(file_name),
            Err(_) => Err(CdError::InvalidUTF8ByteVector),
        }
    }

    fn cd_32_checksum() -> Bytes {
        let byte_vector = hex::decode(Self::CD_32_CHECKSUM).expect("Invalid Hex String");
        let big_endian_bytes = Bytes::from(byte_vector);
        Cd::big_endian_to_little_endian(&big_endian_bytes)
    }

    fn big_endian_to_little_endian(bytes: &Bytes) -> Bytes {
        /*
        The Bytes library does not have a way to convert endian-ness.
        This function implements this.
        */
        let mut little_endian_bytes = Vec::with_capacity(bytes.len());

        for chunk in bytes.chunks_exact(4) {
            let mut int_bytes = [0u8; 4];
            int_bytes.copy_from_slice(chunk);
            let value = u32::from_be_bytes(int_bytes);
            little_endian_bytes.extend(value.to_le_bytes().iter());
        }

        Bytes::from(little_endian_bytes)
    }
}
