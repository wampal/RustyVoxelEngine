use std::fs::File;
use std::io::{self, Read, Write};

/// Запись бинарного файла
pub fn write_binary_file(filename: &str, data: &[u8]) -> io::Result<()> {
    let mut output = File::create(filename)?;
    output.write_all(data)?;
    Ok(())
}

/// Чтение бинарного файла
pub fn read_binary_file(filename: &str, data: &mut [u8]) -> io::Result<usize> {
    let mut input = File::open(filename)?;
    input.read(data)
}
