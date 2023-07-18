const CD_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];
const EXTRA_FIELD_SIGNATURE: [u8; 2] = [0x01, 0x00];

pub struct Cd {
    signature: u32,
    version_made_by: u16,
    version_needed_to_extract: u16,
    general_purpose_bit_flag: u16,
    compression_method: u16,
    file_last_modification_time: u16,
    file_last_modification_date: u16,
    crc_32_hash: u32,
    compressed_size: u64,
    uncompressed_size: u64,
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_number_where_file_starts: u32,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    relative_offset_of_local_file_header: u64,
    file_name: String,
    extra_field: Option<ExtraField>,
    file_comment: Option<String>,
}

pub struct ExtraField {
    signature: u16,
    size_of_extra_field_chunk: u16,
    uncompressed_size: u64,
    compressed_size: u64,
    local_header_offset: u64,
    disk_number_where_file_starts: u32,
}
