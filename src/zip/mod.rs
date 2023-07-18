pub mod cd;
pub mod eocd;

use cd::*;
use eocd::*;

#[derive(Debug)]
pub enum WordSize {
    Bit32,
    Bit64,
}

pub struct Zip {
    eocd: Eocd,
    cd_list: Vec<Cd>,
}

impl Zip {
    pub fn build(bytes: &[u8]) -> Zip {
        Zip {
            eocd: Eocd::from(bytes),
            cd_list: Vec::new(),
        }
    }

    pub fn word_size(&self) -> WordSize {
        match self.eocd.verify_wordsize() {
            eocd::WordSize::Bit32 => WordSize::Bit32,
            eocd::WordSize::Bit64 => WordSize::Bit64,
        }
    }

    pub fn disk_number(&self) -> u32 {
        self.eocd.disk_number()
    }

    pub fn disk_where_cd_starts(&self) -> u32 {
        self.eocd.disk_where_cd_starts()
    }

    pub fn number_of_cd_records_on_disk(&self) -> u32 {
        self.eocd.number_of_cd_records_on_disk()
    }

    pub fn total_number_of_cd_records(&self) -> u32 {
        self.eocd.total_number_of_cd_records()
    }

    pub fn size_of_cd(&self) -> u64 {
        self.eocd.size_of_cd()
    }
}
