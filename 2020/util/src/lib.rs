use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn read_lines<'a, P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .into_iter()
        .filter_map(|line| line.ok()))
}
