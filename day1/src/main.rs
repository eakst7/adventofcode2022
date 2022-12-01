use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

struct GroupedInput {
    lines: Vec<String>
}

fn read(path: &str) -> Vec<GroupedInput> {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut out: Vec<GroupedInput> = Vec::new();
    let mut v: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            out.push(GroupedInput {
                lines: v
            });
            v = Vec::new();
        } else {
            v.push(line);
        }
    }
    out.push(GroupedInput {
        lines: v
    });

    out
}

fn part1() {
    println!("Hello, world!");

    let gi = read("src/input.txt");

    let mut max = 0;
    for g in gi {
        let mut groupsum = 0;
        println!("Group");
        for s in g.lines {
            println!("{}", s);
            let v = s.parse::<i32>().unwrap();
            groupsum += v;
        }
        println!("{}", groupsum);
        if (groupsum > max) {
            max = groupsum;
        }
    }

    println!("Max is {}", max);
}

fn part2() {
    println!("Hello, world!");

    let gi = read("src/input.txt");

    let mut list_of_sums: Vec<i32> = Vec::new();
    for g in gi {
        let mut groupsum = 0;
        println!("Group");
        for s in g.lines {
            println!("{}", s);
            let v = s.parse::<i32>().unwrap();
            groupsum += v;
        }
        println!("{}", groupsum);
        list_of_sums.push(groupsum);
    }

    list_of_sums.sort();
    println!("{:?}", list_of_sums);

    let mut sum = 0;
    for s in &list_of_sums[list_of_sums.len()-3..] {
        println!("{:?}", s);
        sum += s;
    }

    println!("Sum is {}", sum);

}

fn main() {
    part2();
}