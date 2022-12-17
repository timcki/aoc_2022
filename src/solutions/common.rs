use std::{
    fs::File,
    io::{self, Read},
};

#[allow(dead_code)]
pub fn read_file(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
