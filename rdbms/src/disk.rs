const PAGE_SIZE: i32 = 4096;

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }
    pub fn open(heap_file_size: impl AsRef(Path)) -> io::Result<Self> {
        let a = String::from("hoge");
        hoge(s);
        println!(s);
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_size)?;
        Self::new(heap_file);
    }
    pub fn allocate_page(&mut self) -> PageId {}
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {}
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {}
}

fn hoge(s: String) {
    println!("{}", s);
}
