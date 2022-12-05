use std::usize;

use aoclib::reader::lines;

#[allow(dead_code)]
fn init_test() -> Vec<Vec<char>> {
    //     [D]    
    // [N] [C]    
    // [Z] [M] [P]
    //  1   2   3 
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(vec![]);
    stacks.push(vec!['Z', 'N']);
    stacks.push(vec!['M', 'C', 'D']);
    stacks.push(vec!['P']);

    stacks
}

#[allow(dead_code)]
fn init_input() -> Vec<Vec<char>> {
    //             [G] [W]         [Q]    
    // [Z]         [Q] [M]     [J] [F]    
    // [V]         [V] [S] [F] [N] [R]    
    // [T]         [F] [C] [H] [F] [W] [P]
    // [B] [L]     [L] [J] [C] [V] [D] [V]
    // [J] [V] [F] [N] [T] [T] [C] [Z] [W]
    // [G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
    // [R] [J] [S] [Z] [R] [S] [D] [L] [J]
    //  1   2   3   4   5   6   7   8   9 
    
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(vec![]);
    stacks.push(vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z']);
    stacks.push(vec!['J', 'R', 'V', 'L']);
    stacks.push(vec!['S', 'Q', 'F']);
    stacks.push(vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G']);
    stacks.push(vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W']);
    stacks.push(vec!['S', 'W', 'T', 'C', 'H', 'F']);
    stacks.push(vec!['D', 'Z', 'C', 'D', 'F', 'N', 'J']);
    stacks.push(vec!['L', 'G', 'Z', 'V', 'W', 'R', 'F', 'Q']);
    stacks.push(vec!['J', 'B', 'W', 'D', 'P']);

    stacks
}

pub fn part2() -> String {
    // let mut stacks: Vec<Vec<char>> = init_test();
    // let inputfile = "src/test.txt";

    let mut stacks: Vec<Vec<char>> = init_input();
    let inputfile = "src/input.txt";

    //println!("{:?}", stacks);
    for line in lines(inputfile) {
        //println!("{}", line);
        let mut i = line.splitn(6," ");
        i.next();
        let count = i.next().unwrap().parse::<usize>().unwrap();
        i.next();
        let from = i.next().unwrap().parse::<usize>().unwrap();
        i.next();
        let to = i.next().unwrap().parse::<usize>().unwrap();

        let idx = stacks.get(from).unwrap().len() - count;
        let mut v2 = stacks.get_mut(from).unwrap().split_off(idx);
        stacks.get_mut(to).unwrap().append(&mut v2);

        //println!("{:?}", stacks);
    }

    let mut r : String = String::new();
    for s in stacks {
        let c = s.last();
        match c {
            Some(c) => {
                r.push(*c);
            },
            _ => { (); }
        };
    }

    return r;
}