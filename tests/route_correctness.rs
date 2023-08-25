use std::time::{Duration, Instant};

use common::fmi_reader::GraphFileReader;
use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;
mod common;

const GRAPH_FILE: &str = "tests/data/stgtregbz.fmi";
const TEST_FILE: &str = "tests/data/stgtregbz_test.txt";

#[test]
fn test_route_correctness() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let mut contractor = Contractor::new(graph.clone());
    let _shortcuts = contractor.contract();

    let dijskstra = ChDijsktra::new(contractor.get_graph());

    let mut times = Vec::new();

    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
    for test in &test_cases {
        let before = Instant::now();
        let route = dijskstra.single_pair_shortest_path(test.source, test.target);
        times.push(before.elapsed());

        //println!(
        //    "{} {} {}",
        //    route.cost.unwrap() as i32 - test.cost,
        //    route.cost.unwrap(),
        //    test.cost
        //);

        // test sum of cost
        assert_eq!(route.cost.unwrap() as i32, test.cost);

        // test sum of edge cost
        let mut all_cost = 0;
        for edge in &route.route {
            all_cost += edge.cost;
        }
        assert_eq!(
            all_cost as i32, test.cost,
            "sum of edges costs is not correct"
        );

        // test edges are continuous
        // for edge_window in route.route.windows(2) {
        //     assert_eq!(
        //         edge_window[0].target, edge_window[1].source,
        //         "current edges source doesn't match previous edges target"
        //     );
        // }
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
