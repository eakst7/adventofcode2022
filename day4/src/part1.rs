use bit_set::BitSet;

use crate::reader::lines;
use crate::bitsetrange::SetRange;

#[allow(dead_code)]
pub fn part1(input: &str) {
    let mut count = 0;
    for line in lines(input).map(|l| l.unwrap()) {
        let (e1,e2) = line.split_once(",").unwrap();
        let (e1start,e1end) = e1.split_once("-").unwrap();
        let (e2start,e2end) = e2.split_once("-").unwrap();

        let e1start: usize = e1start.parse::<usize>().unwrap();
        let e1end: usize = e1end.parse::<usize>().unwrap();
        let e2start: usize = e2start.parse::<usize>().unwrap();
        let e2end: usize = e2end.parse::<usize>().unwrap();

        let mut e1bs: BitSet = BitSet::new();
        e1bs.set_range(e1start, e1end);

        let mut e2bs: BitSet = BitSet::new();
        e2bs.set_range(e2start, e2end);

        //println!("{} {:?} {:?} {} {}", line, e1bs, e2bs, e1bs.is_subset(&e2bs), e2bs.is_subset(&e2bs));

        if e1bs.is_subset(&e2bs) || e2bs.is_subset(&e1bs) {
            count += 1;
        }
    }

    println!("Part1: {}", count);
}