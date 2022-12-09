mod part1;
mod part2;

#[allow(unused_imports)]
use part1::part1;
use part2::part2;

fn main() {
    println!("Part1: {}", part1("src/input.txt"));
    println!("Part2: {}", part2("src/input.txt"));
}

