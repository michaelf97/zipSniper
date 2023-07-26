use std::fmt;

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
    pub eocd: Eocd,
    pub cd_list: Vec<Cd>,
}

impl fmt::Display for Zip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cd in &self.cd_list {
            println!("{}\n", cd);
        }
        println!("{}", self.eocd);
        write!(f, "Done")
    }
}

impl Zip {
    pub fn build(eocd: Eocd, cd_list: Vec<Cd>) -> Zip {
        Zip { eocd, cd_list }
    }

    pub fn word_size(&self) -> WordSize {
        match self.eocd.verify_wordsize() {
            eocd::WordSize::Bit32 => WordSize::Bit32,
            eocd::WordSize::Bit64 => WordSize::Bit64,
        }
    }
}
