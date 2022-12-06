mod part1;
mod part2;

#[allow(unused_imports)]
use part1::part1;
use part2::part2;

fn main() {
    println!("Hello, world!");
    // println!("Part1: {}", part1("src/test1.txt"));
    // println!("Part1: {}", part1("src/test2.txt"));
    // println!("Part1: {}", part1("src/test3.txt"));
    // println!("Part1: {}", part1("src/test4.txt"));
    // println!("Part1: {}", part1("src/test5.txt"));
    println!("Part1: {}", part1("src/input.txt"));

    // println!("Part2: {}", part2("src/test1.txt"));
    // println!("Part2: {}", part2("src/test2.txt"));
    // println!("Part2: {}", part2("src/test3.txt"));
    // println!("Part2: {}", part2("src/test4.txt"));
    // println!("Part2: {}", part2("src/test5.txt"));
    println!("Part2: {}", part2("src/input.txt"));
}
