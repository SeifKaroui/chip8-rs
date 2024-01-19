use std::fs::File;
use std::io::prelude::*;

use super::MEMORY_SIZE;

const MAX_ROM_SIZE: usize = MEMORY_SIZE - 0x200;

pub struct RomLoader {
    pub rom: [u8; MAX_ROM_SIZE],
}

impl RomLoader {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("File not found");
        let mut buffer = [0u8; MAX_ROM_SIZE];

        f.read(&mut buffer).expect("Couldn't read file");

        RomLoader { rom: buffer }
    }
}
