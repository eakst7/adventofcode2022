use aoclib::reader::lines;
use std::collections::VecDeque;

fn no_repeats(vd: &VecDeque<char>) -> bool {
    for i in 0..vd.len() {
        for j in i+1..vd.len() {
            if vd[i] == vd[j] { return false; }
        }
    }

    return true;
}

pub fn part2(input: &str) -> i32 {
    let line = lines(input).next().unwrap();

    let mut index = 0;
    let mut last14: VecDeque<char> = VecDeque::with_capacity(14);
    for c in line.chars().take(14) {
        last14.push_back(c);
    }

    for (i,b) in line.chars().enumerate().skip(14) {
        last14.rotate_left(1);
        last14[13] = b;
        if no_repeats(&last14) {
            index = i+1;
            break;
        }
    }

    index as i32
}