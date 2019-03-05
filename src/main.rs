mod cpu;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();

    let pif_rom = read_bin(pif_file_name);
    let rom     = read_bin(rom_file_name);

    let mut cpu = cpu::Cpu::default();
    cpu.power_on_reset();
    println!("{:#?}", &cpu);
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file     = File::open(path.as_ref()).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
