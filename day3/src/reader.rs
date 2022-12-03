use std::fs::File;
use std::path::Path;
use std::io::BufReader;
pub use std::io::BufRead;

pub fn lines(path: &str) -> std::io::Lines<BufReader<File>> {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}