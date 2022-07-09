mod rom;
fn main() {
    let rom_path = std::env::args().nth(1).unwrap();
    let mut rom_file = std::fs::File::open(rom_path).unwrap();
    let rom = rom::Rom::new(&mut rom_file).unwrap();
    println!("{:?}", rom.get_header());
}
