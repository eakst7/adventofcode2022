mod part1;
mod part2;

#[allow(unused_imports)]
use aoclib::reader::lines;

#[allow(unused_imports)]
use part1::part1;
use part2::part2;

fn main() {
    println!("Hello, world!");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
