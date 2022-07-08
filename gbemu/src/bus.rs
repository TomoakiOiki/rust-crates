pub struct Bus {
    pub bank: [u8; 0x7FFF],
    pub vram: [u8; 0x2000],
    pub external_ram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7F],
    pub interrupt_enable: u8,
}

impl Bus {
    pub fn new() {}

    pub fn read(addr: u8) -> u8 {
        match addr {
            0x00..=0x3FFF => self.bank[addr as usize],
            0x4000..=0x7FFF => self.bank[addr as usize],
            0x8000..=0x9FFF => self.vram[addr as usize],
            0xA000..=0xBFFF => self.external_ram[addr as usize],
            0xC000..=0xDFFF => self.wram[addr as usize],
            0xE000..=0xFDFF => self.wram[addr as usize], // prohibited to write here
            0xFE00..=0xFE9F => self.oam[addr as usize],
            // 0xFEA0..=0xFEFF => unused,
            0xFF00..=0xFF7F => self.io[addr as usize],
            0xFF80..=0xFFFE => self.hram[addr as usize],
            0xFFFF => self.interrupt_enable,
            _ => panic!("Invalid address: {:02X}", addr),
        }
    }
}
