use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TestRoute {
    pub source: u32,
    pub target: u32,
    pub cost: i32,
}

pub fn get_test_cases(filename: &str) -> Vec<TestRoute> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split_line = line.split_whitespace();
            let source: u32 = split_line.next().unwrap().parse().unwrap();
            let target: u32 = split_line.next().unwrap().parse().unwrap();
            let cost: i32 = split_line.next().unwrap().parse().unwrap();
            TestRoute {
                source,
                target,
                cost,
            }
        })
        .collect()
}
