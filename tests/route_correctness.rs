use rand::Rng;
use route_planner::dijkstra::ch_dijkstra::ChDijsktra;
use route_planner::dijkstra::dijkstra_helper::DijkstraHelper;
use route_planner::heuristic::heuristic::Heuristic;
use route_planner::heuristic::landmark_heuristic::LandmarkHeuristic;
use std::rc::Rc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use common::fmi_reader::GraphFileReader;
use route_planner::contrator::Contractor;
use route_planner::graph::bidirectional_graph::BidirectionalGraph;

mod common;

const GRAPH_FILE: &str = "tests/data/stgtregbz.fmi";
const TEST_FILE: &str = "tests/data/stgtregbz_test.txt";

#[test]
fn test_landmark() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);
    let number_nodes = graph.outgoing_edges.len();

    let i = 150;
    let heuristic = LandmarkHeuristic::new(&graph, i);

    let graph = RwLock::new(graph);
    let graph = Rc::new(graph);
    let dijskstra = DijkstraHelper::new(graph);

    let mut rng = rand::thread_rng();
    let mut lower_percentages = Vec::new();
    let mut upper_percentages = Vec::new();

    for _ in 0..1000 {
        let source = rng.gen_range(0..number_nodes) as u32;
        let target = rng.gen_range(0..number_nodes) as u32;
        //let heuristic = heuristic_org.tune(source, target);
        let lower_bound = heuristic.lower_bound(source, target).unwrap_or(0);
        let true_cost = dijskstra.single_pair(source, target).unwrap_or(0);
        let upper_bound = heuristic.upper_bound(source, target).unwrap_or(0);

        let mut lower_percentage = 100.0 - (lower_bound as f32 / (true_cost as f32 / 100.0));
        if lower_percentage.is_nan() {
            lower_percentage = 0.0;
        }
        lower_percentages.push(lower_percentage);

        let mut upper_percentage = (upper_bound as f32 / (true_cost as f32 / 100.0)) - 100.0;
        if upper_percentage.is_nan() {
            upper_percentage = 0.0;
        }
        upper_percentages.push(upper_percentage);

        assert!(lower_bound <= true_cost);
        assert!(true_cost <= upper_bound);
    }

    let lower: f32 = lower_percentages.iter().sum();
    let lower = lower / (lower_percentages.len() as f32);

    let upper: f32 = upper_percentages.iter().sum();
    let upper = upper / (upper_percentages.len() as f32);
    println!("{} {} {}", i, lower, upper);
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
        }
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
