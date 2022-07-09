use hex;
use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek},
};

#[derive(Debug)]
pub struct Rom {
    header: RomHeader,
}

impl Rom {
    pub fn new(rom_file: &mut File) -> io::Result<Self> {
        Ok(Self {
            header: RomHeader::new(rom_file)?,
        })
    }

    pub fn get_header(&self) -> &RomHeader {
        &self.header
    }
}
pub struct RomHeader {
    entry_point: [u8; 4],       // 0x0100-0x0103
    logo: [u8; 48],             // 0x0104-0x0133
    title: [u8; 16],            // 0x0134-0x0143 includes manucaturer code and cgb flag
    new_licensee_code: [u8; 2], // 0x0144-0x0145
    cgb_flag: [u8; 1],          // 0x0146
    cartridge_type: [u8; 1],    // 0x0147
    rom_size: [u8; 1],          // 0x0148
    ram_size: [u8; 1],          // 0x0149
    destination_code: [u8; 1],  // 0x014A
    old_licensee_code: [u8; 1], // 0x014B
    mask_rom_version: [u8; 1],  // 0x014C
    header_checksum: [u8; 1],   // 0x014D
    global_checksum: [u8; 2],   // 0x014E-0x014F
}

impl RomHeader {
    pub fn default() -> Self {
        Self {
            entry_point: [0x00; 4],
            logo: [0x00; 48],
            title: [0x00; 16],
            new_licensee_code: [0x00; 2],
            cgb_flag: [0x00; 1],
            cartridge_type: [0x00; 1],
            rom_size: [0x00; 1],
            ram_size: [0x00; 1],
            destination_code: [0x00; 1],
            old_licensee_code: [0x00; 1],
            mask_rom_version: [0x00; 1],
            header_checksum: [0x00; 1],
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
            .field("new_licensee_code", &hex::encode(&self.new_licensee_code))
            .field("cgb_flag", &hex::encode(&self.cgb_flag))
            .field("cartridge_type", &hex::encode(&self.cartridge_type))
            .field("rom_size", &hex::encode(&self.rom_size))
            .field("ram_size", &hex::encode(&self.ram_size))
            .field("destination_code", &hex::encode(&self.destination_code))
            .field("old_licensee_code", &hex::encode(&self.old_licensee_code))
            .field("mask_rom_version", &hex::encode(&self.mask_rom_version))
            .field("header_checksum", &hex::encode(&self.header_checksum))
            .field("global_checksum", &hex::encode(&self.global_checksum))
            .finish()
    }
}

pub fn read_rom_header(rom_file: &mut File) -> io::Result<RomHeader> {
    let mut rom_header = RomHeader::default();
    rom_file.seek(io::SeekFrom::Start(0x100 as u64))?;
    rom_file.read_exact(&mut rom_header.entry_point)?;
    rom_file.read_exact(&mut rom_header.logo)?;
    rom_file.read_exact(&mut rom_header.title)?;
    rom_file.read_exact(&mut rom_header.new_licensee_code)?;
    rom_file.read_exact(&mut rom_header.cgb_flag)?;
    rom_file.read_exact(&mut rom_header.cartridge_type)?;
    rom_file.read_exact(&mut rom_header.rom_size)?;
    rom_file.read_exact(&mut rom_header.ram_size)?;
    rom_file.read_exact(&mut rom_header.destination_code)?;
    rom_file.read_exact(&mut rom_header.old_licensee_code)?;
    rom_file.read_exact(&mut rom_header.mask_rom_version)?;
    rom_file.read_exact(&mut rom_header.header_checksum)?;
    rom_file.read_exact(&mut rom_header.global_checksum)?;
    // println!("{:?}", rom_header);
    Ok(rom_header)
}
