use hex;
use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek},
};

#[derive(Debug)]
pub struct Rom {
    header: RomHeader,
    bank: Vec<u8>,
}

impl Rom {
    pub fn new(rom_file: &mut File) -> io::Result<Self> {
        Ok(Self {
            header: RomHeader::new(rom_file)?,
            bank: read_rom_bank(rom_file)?,
        })
    }

    pub fn get_header(&self) -> &RomHeader {
        &self.header
    }
}

pub struct RomHeader {
    entry_point: [u8; 4],       // 0x0100-0x0103
    logo: [u8; 48],             // 0x0104-0x0133
    title: [u8; 11],            // 0x0134-0x013E
    manufacturer_code: [u8; 4], // 0x013F-0x0142
    cgb_flag: bool,             // 0x0143
    new_licensee_code: [u8; 2], // 0x0144-0x0145
    sgb_flag: bool,             // 0x0146
    cartridge_type: usize,      // 0x0147
    rom_size: RomSize,          // 0x0148
    ram_size: usize,            // 0x0149
    destination_code: usize,    // 0x014A
    old_licensee_code: usize,   // 0x014B
    mask_rom_version: usize,    // 0x014C
    header_checksum: usize,     // 0x014D
    global_checksum: [u8; 2],   // 0x014E-0x014F
}

impl RomHeader {
    pub fn default() -> Self {
        Self {
            entry_point: [0x00; 4],
            logo: [0x00; 48],
            title: [0x00; 11],
            manufacturer_code: [0x00; 4],
            cgb_flag: false,
            new_licensee_code: [0x00; 2],
            sgb_flag: false,
            cartridge_type: 0,
            rom_size: RomSize::default(),
            ram_size: 0,
            destination_code: 0,
            old_licensee_code: 0,
            mask_rom_version: 0,
            header_checksum: 0,
            global_checksum: [0x00; 2],
        }
    }

    pub fn new(rom_file: &mut File) -> io::Result<Self> {
        let header = read_rom_header(rom_file)?;
        Ok(header)
    }
}

impl fmt::Debug for RomHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rom Header")
            .field("entry_point", &hex::encode(&self.entry_point))
            .field("logo", &hex::encode(&self.logo))
            .field("title", &hex::encode(&self.title))
            .field("manufacturer_code", &hex::encode(&self.manufacturer_code))
            .field("cgb_flag", &self.cgb_flag)
            .field("new_licensee_code", &hex::encode(&self.new_licensee_code))
            .field("cgb_flag", &self.sgb_flag)
            .field("cartridge_type", &self.cartridge_type)
            .field("rom_size", &self.rom_size)
            .field("ram_size", &self.ram_size)
            .field("destination_code", &self.destination_code)
            .field("old_licensee_code", &self.old_licensee_code)
            .field("mask_rom_version", &self.mask_rom_version)
            .field("header_checksum", &self.header_checksum)
            .field("global_checksum", &self.global_checksum)
            .finish()
    }
}

#[derive(Debug)]
pub struct RomSize {
    size: usize,
    num_of_banks: usize,
}

impl RomSize {
    pub fn default() -> Self {
        Self {
            size: 0,
            num_of_banks: 0,
        }
    }
}

pub fn read_rom_header(rom: &mut File) -> io::Result<RomHeader> {
    let mut rom_header = RomHeader::default();
    rom.seek(io::SeekFrom::Start(0x100 as u64))?;
    rom.read_exact(&mut rom_header.entry_point)?;
    rom.read_exact(&mut rom_header.logo)?;
    rom.read_exact(&mut rom_header.title)?;
    rom.read_exact(&mut rom_header.manufacturer_code)?;

    rom_header.cgb_flag = match rom.take(1).bytes().next() {
        Some(Ok(0x80 | 0xC0)) => true,
        _ => false,
    };

    rom.read_exact(&mut rom_header.new_licensee_code)?;

    rom_header.sgb_flag = match rom.take(1).bytes().next() {
        Some(Ok(0x00)) => false,
        Some(Ok(0x03)) => true,
        _ => panic!("Unknown SGB flag"),
    };

    rom_header.cartridge_type = rom.take(1).bytes().next().unwrap()?.into();

    let mut rom_size = RomSize::default();
    match rom.take(1).bytes().next() {
        Some(Ok(n @ 0x00..=0x08)) => {
            rom_size.size = ((32 * 1024) << n) as usize;
            rom_size.num_of_banks = (2 << n) as usize;
        }
        Some(Ok(0x52)) => {
            rom_size.size = (1.1 * 1024.0 * 1024.0) as usize;
            rom_size.num_of_banks = 72 as usize;
        }
        Some(Ok(0x53)) => {
            rom_size.size = (1.2 * 1024.0 * 1024.0) as usize;
            rom_size.num_of_banks = 80 as usize;
        }
        Some(Ok(0x54)) => {
            rom_size.size = (1.5 * 1024.0 * 1024.0) as usize;
            rom_size.num_of_banks = 96 as usize;
        }
        _ => panic!("Invalid ROM size"),
    }
    rom_header.rom_size = rom_size;

    rom_header.ram_size = match rom.take(1).bytes().next() {
        Some(Ok(0x00)) => 0 as usize,
        Some(Ok(0x01)) => (2 * 1024 * 1024) as usize,
        Some(Ok(0x02)) => (8 * 1024 * 1024) as usize,
        Some(Ok(0x03)) => (32 * 1024 * 1024) as usize,
        Some(Ok(0x04)) => (128 * 1024 * 1024) as usize,
        Some(Ok(0x05)) => (64 * 1024 * 1024) as usize,
        _ => panic!("Invalid RAM size"),
    };

    rom_header.destination_code = match rom.take(1).bytes().next() {
        Some(Ok(0x00)) => 0,
        Some(Ok(0x01)) => 1,
        _ => panic!("Invalid destination code"),
    };

    rom_header.old_licensee_code = rom.take(1).bytes().next().unwrap()?.into();
    rom_header.mask_rom_version = rom.take(1).bytes().next().unwrap()?.into();
    rom_header.header_checksum = rom.take(1).bytes().next().unwrap()?.into();
    rom.read_exact(&mut rom_header.global_checksum)?;
    // println!("{:?}", rom_header);

    if calc_header_checksum(rom).unwrap() != rom_header.header_checksum {
        panic!("Invalid header checksum");
    }

    Ok(rom_header)
}

fn calc_header_checksum(rom: &mut File) -> io::Result<usize> {
    let mut x: u8 = 0;
    rom.seek(io::SeekFrom::Start(0x134 as u64))?;
    for _ in 0x134..=0x14C {
        let byte = rom.take(1).bytes().next().unwrap()?;
        x = x.wrapping_sub(byte + 1)
    }
    Ok(x as usize)
}

pub fn read_rom_bank(rom_file: &mut File) -> io::Result<Vec<u8>> {
    let mut bank = Vec::new();
    rom_file.seek(io::SeekFrom::Start(0x150 as u64))?;
    rom_file.read_to_end(&mut bank)?;
    println!("{}", bank.len() / 1024);
    Ok(bank)
}
