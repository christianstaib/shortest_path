use common::fmi_reader::GraphFileReader;
use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;
mod common;

const GRAPH_FILE: &str = "data/stgtregbz.fmi";
const TEST_FILE: &str = "data/stgtregbz_test.txt";

#[test]
fn test_route_correctness() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let mut contractor = Contractor::new(graph);
    contractor.contract();
    let graph = contractor.graph;

    let dijskstra = ChDijsktra::new(graph);

    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
    for test in &test_cases {
        let cost = dijskstra.single_pair_shortest_path(test.source, test.target);
        assert_eq!(cost as i32, test.cost);
    }
}
