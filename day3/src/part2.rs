use crate::reader::lines;
use crate::part1::to_bitset;
use itertools::Itertools;

#[allow(dead_code)]
pub fn part2(input: &str) {
    let mut sum = 0;
    
    for (e1, e2, e3) in lines(input).map(|l| l.unwrap()).tuples() {
        println!("{} / {} / {}", e1, e2, e3);

        let mut bs1 = to_bitset(&e1);
        let bs2 = to_bitset(&e2);
        let bs3 = to_bitset(&e3);

        bs1.intersect_with(&bs2);
        bs1.intersect_with(&bs3);

        for i in &bs1 {
            sum += i;
            println!("{}", i);
        }
    }

    println!("{}", sum);
}