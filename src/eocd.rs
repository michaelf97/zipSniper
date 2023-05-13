use bytes::{Buf, Bytes};

#[derive(Debug)]
pub struct Eocd {
    binary: Bytes,
    pub word_size: WordSize,
}

#[derive(Debug, PartialEq)]
pub enum WordSize {
    Bit32,
    Bit64,
}

struct FieldMetaData {
    size: usize,
    offset: usize,
}

#[derive(Debug)]
pub enum EocdError {
    NotValidBinary,
    UnknownWordSize,
    AttributeNotPresent,
    NotImplemented,
}

impl FieldMetaData {
    fn slice_range(&self) -> std::ops::Range<usize> {
        self.offset..(self.offset + self.size)
    }
}

impl Eocd {
    /*
    These hex strings represent the start of the "End of Central Directory
    " (EOCD) record.
    */
    const EOCD_32_CHECKSUM: &str = "06054b50";
    const EOCD_64_CHECKSUM: &str = "06064b50";

    pub fn from(binary: Bytes) -> Self {
        Self {
            word_size: Self::verify(&binary).unwrap(),
            binary,
        }
    }

    pub fn number_of_this_disk(&self) -> u32 {
        /*
        Description: Number of this disk
        32-Bit offset: 4
        32-Bit size: 2
        64-Bit offset: 16
        64-Bit size: 4
        */
        let metadata_32 = FieldMetaData { size: 2, offset: 4 }.slice_range();
        let metadata_64 = FieldMetaData {
            size: 4,
            offset: 16,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u16_le() as u32,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u32_le(),
        }
    }

    pub fn disk_where_cd_starts(&self) -> u32 {
        /*
        Description: Disk where the Central Directory starts
        32-Bit offset: 6
        32-Bit size: 2
        64-Bit offset: 20
        64-Bit size: 4
        */
        let metadata_32 = FieldMetaData { size: 2, offset: 6 }.slice_range();
        let metadata_64 = FieldMetaData {
            size: 4,
            offset: 20,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u16_le() as u32,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u32_le(),
        }
    }

    pub fn number_of_central_directory_records_on_this_disk(&self) -> u64 {
        /*
        Description: Number of central directory records on this disk
        32-Bit offset: 8
        32-Bit size: 2
        64-Bit offset: 24
        64-Bit size: 8
        */
        let metadata_32 = FieldMetaData { size: 2, offset: 8 }.slice_range();
        let metadata_64 = FieldMetaData {
            size: 8,
            offset: 24,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u16_le() as u64,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u64_le(),
        }
    }

    pub fn total_number_of_central_directory_records(&self) -> u64 {
        /*
        Description: Total number of central directory records
        32-Bit offset: 10
        32-Bit size: 2
        64-Bit offset: 32
        64-Bit size: 8
        */
        let metadata_32 = FieldMetaData {
            size: 2,
            offset: 10,
        }
        .slice_range();
        let metadata_64 = FieldMetaData {
            size: 8,
            offset: 32,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u16_le() as u64,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u64_le(),
        }
    }

    pub fn size_of_central_directory(&self) -> u64 {
        /*
        Description: Size of central directory
        32-Bit offset: 12
        32-Bit size: 4
        64-Bit offset: 40
        64-Bit size: 8
        */
        let metadata_32 = FieldMetaData {
            size: 4,
            offset: 12,
        }
        .slice_range();
        let metadata_64 = FieldMetaData {
            size: 8,
            offset: 40,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u32_le() as u64,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u64_le(),
        }
    }

    pub fn offset_of_start_of_central_directory(&self) -> u64 {
        /*
        Description: Size of central directory
        32-Bit offset: 16
        32-Bit size: 4
        64-Bit offset: 48
        64-Bit size: 8
        */
        let metadata_32 = FieldMetaData {
            size: 4,
            offset: 16,
        }
        .slice_range();
        let metadata_64 = FieldMetaData {
            size: 8,
            offset: 48,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u32_le() as u64,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u64_le(),
        }
    }

    pub fn zip_file_comment_length(&self) -> u64 {
        /*
        Description: Comment length
        32-Bit offset: 20
        32-Bit size: 2
        */
        let metadata_32 = FieldMetaData {
            size: 2,
            offset: 20,
        }
        .slice_range();
        let metadata_64 = FieldMetaData { size: 8, offset: 4 }.slice_range();

        match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).get_u16_le() as u64,
            WordSize::Bit64 => self.binary.slice(metadata_64).get_u64_le() - 44,
        }
    }

    pub fn size_of_eocd64_minus_12(&self) -> Result<u64, EocdError> {
        /*
        Description: Size of the EOCD64 minus 12
        64-Bit offset: 4
        64-Bit size: 8
        */
        let metadata_64 = FieldMetaData { size: 8, offset: 4 }.slice_range();

        match self.word_size {
            WordSize::Bit32 => Err(EocdError::AttributeNotPresent),
            WordSize::Bit64 => Ok(self.binary.slice(metadata_64).get_u64_le()),
        }
    }

    pub fn version_made_by(&self) -> Result<u16, EocdError> {
        /*
        Description: Version made by
        64-Bit offset: 12
        64-Bit size: 2
        */
        let metadata_64 = FieldMetaData {
            size: 2,
            offset: 12,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => Err(EocdError::AttributeNotPresent),
            WordSize::Bit64 => Ok(self.binary.slice(metadata_64).get_u16_le()),
        }
    }

    pub fn minimun_version_needed_to_extract(&self) -> Result<u16, EocdError> {
        /*
        Description: Version needed to extract (minimum)
        64-Bit offset: 14
        64-Bit size: 2
        */
        let metadata_64 = FieldMetaData {
            size: 2,
            offset: 14,
        }
        .slice_range();

        match self.word_size {
            WordSize::Bit32 => Err(EocdError::AttributeNotPresent),
            WordSize::Bit64 => Ok(self.binary.slice(metadata_64).get_u16_le()),
        }
    }

    pub fn comment(&self) -> Result<String, EocdError> {
        /*
        Description: Comment
        32-Bit offset: 22
        32-Bit size: n
        64-Bit offset: 56
        64-Bit size: n
        */
        // We need the comment length to calculate "n"
        let comment_length = self.zip_file_comment_length();

        let metadata_32 = FieldMetaData {
            size: comment_length as usize,
            offset: 22,
        }
        .slice_range();
        let metadata_64 = FieldMetaData {
            size: comment_length as usize,
            offset: 56,
        }
        .slice_range();

        let bytes_vec = match self.word_size {
            WordSize::Bit32 => self.binary.slice(metadata_32).to_vec(),
            WordSize::Bit64 => self.binary.slice(metadata_64).to_vec(),
        };
        Ok(String::from_utf8(bytes_vec).expect("Invalid Comment"))
    }

    fn verify(binary: &Bytes) -> Result<WordSize, EocdError> {
        /*
        Description: The checksum to signify the start of the EOCD section.
        Offset: 0
        Size: 4
        */
        let metadata = FieldMetaData { size: 4, offset: 0 }.slice_range();

        let first_four_bytes = binary.slice(metadata);
        if first_four_bytes == Eocd::eocd_32_checksum() {
            return Ok(WordSize::Bit32);
        } else if first_four_bytes == Eocd::eocd_64_checksum() {
            return Ok(WordSize::Bit64);
        }

        Err(EocdError::NotValidBinary)
    }

    fn eocd_32_checksum() -> Bytes {
        let byte_vector = hex::decode(Self::EOCD_32_CHECKSUM).expect("Invalid hex string");
        let big_endian_bytes = Bytes::from(byte_vector);
        Eocd::big_endian_to_little_endian(&big_endian_bytes)
    }

    fn eocd_64_checksum() -> Bytes {
        let byte_vector = hex::decode(Self::EOCD_64_CHECKSUM).expect("Invalid hex string");
        let big_endian_bytes = Bytes::from(byte_vector);
        Eocd::big_endian_to_little_endian(&big_endian_bytes)
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
