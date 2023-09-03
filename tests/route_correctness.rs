use itertools::Itertools;
use rand::Rng;
use route_planner::graph::Edge;
use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

use common::fmi_reader::GraphFileReader;
use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;

use crate::common::test_file_reader::TestRoute;
mod common;

const GRAPH_FILE: &str = "tests/data/stgtregbz.fmi";
const TEST_FILE: &str = "tests/data/stgtregbz_test.txt";

//#[test]
fn test_usa_speed() {
    let file = File::open("tests/data/USA-road-d.USA.gr").unwrap();
    let reader = io::BufReader::new(file);
    let mut graph = BidirectionalGraph::new();
    let mut num_nodes = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        line.next();
        let source = line.next().unwrap().parse().unwrap();
        let target = line.next().unwrap().parse().unwrap();
        let cost = line.next().unwrap().parse().unwrap();
        graph.add_edge(Edge {
            source,
            target,
            cost,
        });

        num_nodes = max(num_nodes, max(source, target));
    });

    let before = Instant::now();
    let mut contractor = Contractor::new(graph);
    contractor.contract();
    println!("contracting graph took {:?}", before.elapsed());
    let graph = contractor.get_graph().unwrap();
    let dijskstra = ChDijsktra::new(graph);

    let mut rng = rand::thread_rng();
    let times: Vec<Duration> = (0..1_000)
        .map(|_| {
            let test = TestRoute {
                source: rng.gen_range(0..num_nodes),
                target: rng.gen_range(0..num_nodes),
                cost: 0,
            };
            let before = Instant::now();
            dijskstra.single_pair_shortest_path(test.source, test.target);
            before.elapsed()
        })
        .collect();

    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}

#[test]
fn test_route_correctness() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let before = Instant::now();
    let mut contractor = Contractor::new(graph);

    let shortcuts = contractor.contract();
    println!("there are {} shortcuts", shortcuts.len());
    let graph = contractor.get_graph().unwrap();

    let dijskstra = ChDijsktra::new(graph);

    println!(
        "contracting graph took {:?}, there are {} shortcuts",
        before.elapsed(),
        shortcuts.len()
    );
    let mut times = Vec::new();

    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
    for _ in 0..1_000 {
        for test in &test_cases {
            let before = Instant::now();
            let route = dijskstra.single_pair_shortest_path(test.source, test.target);
            times.push(before.elapsed());

            let cost = match route.cost {
                Some(cost) => cost as i32,
                None => -1,
            };

            // test sum of cost
            assert_eq!(
                cost, test.cost,
                "cost {} -> {} should be {} but is {}",
                test.source, test.target, test.cost, cost
            );

            //for (prev, next) in route.route.into_iter().tuples() {
            //    if shortcuts
            //        .iter()
            //        .find(|edge| (edge.source == prev) & (edge.target == next))
            //        .is_some()
            //    {
            //        println!("{} {} is shortcut", prev, next);
            //    } else {
            //        println!("{} {} is no shortcut", prev, next);
            //    }
            //}

            // // test sum of edge cost
            // let mut all_cost = 0;
            // for edge in &route.route {
            //     all_cost += edge.cost;
            // }
            // assert_eq!(
            //     all_cost as i32, test.cost,
            //     "sum of edges costs is not correct"
            // );

            // test edges are continuous
            // for edge_window in route.route.windows(2) {
            //     assert_eq!(
            //         edge_window[0].target, edge_window[1].source,
            //         "current edges source doesn't match previous edges target"
            //     );
            // }
        }
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
