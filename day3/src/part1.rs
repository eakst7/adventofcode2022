use crate::reader::lines;
use bit_set::BitSet;

pub fn to_bitset(pocket: &str) -> BitSet {
    let mut bs = BitSet::new();
    let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in pocket.chars() {
        let v = s.find(c).unwrap() + 1;
        bs.insert(v);
    }

    bs
}

#[allow(dead_code)]
pub fn part1(input: &str) {
    let mut sum = 0;
    for line in lines(input).map(|l| l.unwrap()) {
        let (pocket1,pocket2) = &line.split_at(&line.len()/2);

        println!("{} / {}", pocket1, pocket2);
        let mut bs1 = to_bitset(pocket1);
        let bs2 = to_bitset(pocket2);
        bs1.intersect_with(&bs2);

        for i in &bs1 {
            sum += i;
            println!("{}", i);
        }
    }
    println!("{}", sum);
}