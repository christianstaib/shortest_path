use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;
use route_planner::fmi_reader::GraphFileReader;
use route_planner::tests::*;

const GRAPH_FILE: &str = "data/stgtregbz.fmi";
const SOLL_FILE: &str = "benchs/stgtregbz.sol";
const QUEUE_FILE: &str = "benchs/stgtregbz.que";

#[test]
fn main() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let mut contractor = Contractor::new(graph);
    contractor.contract();
    let graph = contractor.graph;

    let dijskstra = ChDijsktra::new(graph);

    let test_cases = get_test_cases(QUEUE_FILE, SOLL_FILE);
    for test in &test_cases {
        let cost = dijskstra.single_pair_shortest_path(test.source, test.target);

        assert_eq!(cost as i32, test.cost);
    }
}
