use memmap::MmapOptions;
use memmap::Mmap;
use std::fs::File;

pub fn load_rom(rom_path: String) -> std::io::Result<Mmap>{
    let file = File::open(rom_path)?;
    let mmap = unsafe {MmapOptions::new().map(&file)? };
    Ok(mmap)
}
