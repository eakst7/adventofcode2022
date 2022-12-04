use std::fs::File;
use std::path::Path;
use std::io::{ BufReader, Lines };
pub use std::io::BufRead;

// Returns an iterator over the (unwrapped) lines
// of the file pointed to by path.
pub fn lines(path: &str) -> UnwrappedLines {
    let path = Path::new(path);
    let file = File::open(path).unwrap();

    UnwrappedLines {
        lines: BufReader::new(file).lines()
    }
}

// An iterator over the lines of an instance of
// Lines<BufReader<File>> with the lines already
// unwrapped, so you can just do:
// 
// for line in lines("path/to/input") { ... }
// 
// panics on an IO error.
pub struct UnwrappedLines {
    lines: Lines<BufReader<File>>
}

impl Iterator for UnwrappedLines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.lines.next();
        match n {
            None => None,
            Some(i) => {
                match i {
                    Err(_) => panic!("ahh!"),
                    Ok(s) => Some(s)
                }
            }
        }
    }

}