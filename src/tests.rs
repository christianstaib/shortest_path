use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct TestRoute {
    pub from: usize,
    pub to: usize,
    pub cost: i32,
}

pub fn get_test_cases() -> Vec<TestRoute> {
    let file = File::open("benchs/germany2.que").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut from_to: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut iter = line.split_whitespace();
            if let (Some(from), Some(to)) = (iter.next(), iter.next()) {
                if let (Ok(fom), Ok(to)) = (from.parse::<usize>(), to.parse::<usize>()) {
                    from_to.push((fom, to));
                }
            }
        }
    }

    let file = File::open("benchs/germany2.sol").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut soll_vec: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let soll: i32 = line.parse().unwrap();
            soll_vec.push(soll);
        }
    }

    from_to
        .iter()
        .zip(soll_vec.iter())
        .map(|((from, to), cost)| TestRoute {
            from: *from,
            to: *to,
            cost: *cost,
        })
        .collect()
}
