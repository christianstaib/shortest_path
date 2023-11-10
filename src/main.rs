use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

mod dijkstra;
mod graph;
mod queue;
mod tests;
use indicatif::ProgressIterator;
use rand::Rng;

use crate::dijkstra::*;
use crate::graph::*;
use crate::tests::*;

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/network_4m.fmi");
    let end = start.elapsed();
    println!("loading graph file took {:.?}", end);
    println!("graph has {} nodes", graph.nodes.len());
    println!("graph has {} edges", graph.edges.len());

    let mut times: Vec<Duration> = Vec::new();
    let mut rng = rand::thread_rng();
    let test_cases: Vec<TestRoute> = (0..1)
        .map(|_| TestRoute {
            from: rng.gen_range(0..graph.nodes.len()) as u32,
            to: rng.gen_range(0..graph.nodes.len()) as u32,
            cost: 0,
        })
        .collect();

    let mut writer = BufWriter::new(File::create("route.csv").unwrap());
    for test in test_cases.iter().progress() {
        let start_main = Instant::now();
        let (used_edges, cost) = dijkstra(&graph, test.from, test.to);
        let end_main = start_main.elapsed();
        println!("cost is {}km", cost);
        // if cost != u32::MAX {
        //     println!(
        //         "cost is {:>6}km ({}, {} -> {}, {})",
        //         cost / 1_000,
        //         graph.nodes[test.from as usize].latitude,
        //         graph.nodes[test.from as usize].longitude,
        //         graph.nodes[test.to as usize].latitude,
        //         graph.nodes[test.to as usize].longitude
        //     );
        // } else {
        //     println!("no route found");
        // }
        let route = get_route(&graph, test.from, test.to, used_edges);
        if let Some(route) = route {
            let ids: Vec<_> = route
                .edges
                .iter()
                .map(|edge| edge.source_id.to_string())
                .collect();
            writeln!(writer, "{}", ids.join(",")).unwrap();
        }

        // match route {
        //     Some(route) => {
        //         let cost = route.cost as i32;
        //         println!(
        //             "{:>8} -> {:>8} diff: {:01}, time: {:?}",
        //             test.from,
        //             test.to,
        //             cost as i32 - test.cost as i32,
        //             end_main
        //         );
        //     }
        //     None => {
        //         println!("no route found");
        //     }
        // }

        times.push(end_main);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
    writer.flush();
}

pub fn meters_to_radians(meters: f64) -> f64 {
    const EARTH_CIRCUMFERENCE_METERS: f64 = 40_000_000.0;
    meters * ((2.0 * PI) / EARTH_CIRCUMFERENCE_METERS)
}

pub fn radians_to_meter(radians: f64) -> f64 {
    const EARTH_CIRCUMFERENCE_METERS: f64 = 40_000_000.0;
    radians * (EARTH_CIRCUMFERENCE_METERS / (2.0 * PI))
}
