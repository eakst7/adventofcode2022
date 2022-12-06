use aoclib::reader::lines;
use std::collections::VecDeque;

fn no_repeats(vd: &VecDeque<char>) -> bool {
    if vd[0] == vd[1] { return false; }
    if vd[0] == vd[2] { return false; }
    if vd[0] == vd[3] { return false; }
    if vd[1] == vd[2] { return false; }
    if vd[1] == vd[3] { return false; }
    if vd[2] == vd[3] { return false; }

    return true;
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let line = lines(input).next().unwrap();


    let mut index = 0;
    let mut last4: VecDeque<char> = VecDeque::with_capacity(4);
    {
        let mut byte_iter = line.chars();
        last4.push_back(byte_iter.next().unwrap());
        last4.push_back(byte_iter.next().unwrap());
        last4.push_back(byte_iter.next().unwrap());
        last4.push_back(byte_iter.next().unwrap());
    }

    for (i,b) in line.chars().enumerate().skip(4) {
        last4.rotate_left(1);
        last4[3] = b;
        if no_repeats(&last4) {
            index = i+1;
            break;
        }
    }

    index as i32
}