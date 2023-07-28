use common::fmi_reader::GraphFileReader;
use criterion::{criterion_group, criterion_main, Criterion};
use route_planner::bidirectional_graph::BidirectionalGraph;
use route_planner::ch_dijkstra::ChDijsktra;
use route_planner::contrator::Contractor;
mod common;






const GRAPH_FILE: &str = "tests/data/stgtregbz.fmi";
const TEST_FILE: &str = "tests/data/stgtregbz_test.txt";

//fn from_elem(c: &mut Criterion) {
//    let graph_file_reader = GraphFileReader::new();
//    let graph = graph_file_reader.from_file(GRAPH_FILE);
//    let graph = BidirectionalGraph::from_graph(&graph);
//
//    let mut contractor = Contractor::new(graph);
//    contractor.contract();
//    let graph = contractor.graph;
//
//    let dijskstra = ChDijsktra::new(graph);
//
//    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
//
//    let mut group = c.benchmark_group("from_elem");
//    for (i, test) in common::test_file_reader::get_test_cases(TEST_FILE)
//        .iter()
//        .enumerate()
//    {
//        group.bench_with_input(BenchmarkId::from_parameter(i), test, |b, test| {
//            b.iter(|| dijskstra.single_pair_shortest_path(test.source, test.target));
//        });
//    }
//    group.finish();
//}

pub fn criterion_benchmark(c: &mut Criterion) {
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(GRAPH_FILE);
    let graph = BidirectionalGraph::from_graph(&graph);

    let mut contractor = Contractor::new(graph);
    contractor.contract();
    let graph = contractor.graph;

    let dijskstra = ChDijsktra::new(graph);

    let test_cases = common::test_file_reader::get_test_cases(TEST_FILE);
    for test in &test_cases {
        c.bench_function("dijskstra_benchmark", |b| {
            b.iter(|| {
                // The benchmark will measure the time taken for this function call
                dijskstra.single_pair_shortest_path(test.source, test.target);
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
