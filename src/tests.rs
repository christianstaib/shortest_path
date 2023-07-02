use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct TestRoute {
    pub source: usize,
    pub target: usize,
    pub cost: i32,
}

pub fn get_test_cases() -> Vec<TestRoute> {
    let file = File::open("benchs/germany2.que").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut source_target: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut split_line = line.split_whitespace();
            if let (Some(source), Some(target)) = (split_line.next(), split_line.next()) {
                if let (Ok(source), Ok(target)) = (source.parse::<usize>(), target.parse::<usize>())
                {
                    source_target.push((source, target));
                }
            }
        }
    }

    let file = File::open("benchs/germany2.sol").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut costs: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let cost: i32 = line.parse().unwrap();
            costs.push(cost);
        }
    }

    source_target
        .iter()
        .zip(costs.iter())
        .map(|((from, to), cost)| TestRoute {
            source: *from,
            target: *to,
            cost: *cost,
        })
        .collect()
}
