use common::fmi_reader::GraphFileReader;
use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;
mod common;

const GRAPH_FILE: &str = "tests/data/stgtregbz.fmi";
const TEST_FILE: &str = "tests/data/stgtregbz_test.txt";

//#[test]
//fn test_cost_correctness() {
//    let graph_file_reader = GraphFileReader::new();
//    let graph = graph_file_reader.from_file(GRAPH_FILE);
//    let graph = BidirectionalGraph::from_graph(&graph);
//
//    let mut contractor = Contractor::new(graph.clone());
//    contractor.contract();
//    let bidirectional_graph = contractor.graph;
//
//    let dijskstra = ChDijsktra::new(bidirectional_graph);
//
//    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
//    for test in &test_cases {
//        let route = dijskstra.single_pair_shortest_path(test.source, test.target);
//        assert_eq!(route.cost.unwrap() as i32, test.cost);
//    }
//}

#[test]
fn test_route_correctness() {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let mut contractor = Contractor::new(graph.clone());
    let shortcuts = contractor.contract();
    let bidirectional_graph = contractor.graph;

    let dijskstra = ChDijsktra::new(bidirectional_graph);

    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
    for test in &test_cases {
        let route = dijskstra.single_pair_shortest_path(test.source, test.target);
        let mut all_cost = 0;
        for slice in route.route.windows(2) {
            println!("slice is {:?}", slice);
            let test = &graph.outgoing_edges[slice[0] as usize];
            let cost = test.iter().find(|edge| edge.target == slice[1]);
            if cost.is_some() {
                all_cost += cost.unwrap().cost;
            } else {
                let cost = 0;
                shortcuts
                    .iter()
                    .filter(|edge| (edge.source == slice[0]) & (edge.target == slice[1]))
                    .map(|edge| edge.cost)
                    .min()
                    .unwrap_or({
                        println!("shortcut not found");
                        0
                    });
                all_cost += cost;
            }
            println!("all cost {}", all_cost);
        }

        assert_eq!(all_cost as i32, test.cost);
    }
}
