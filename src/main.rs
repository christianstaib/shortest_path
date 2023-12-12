use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    time::{Duration, Instant},
};

use indicatif::ProgressIterator;
use rand::Rng;
use route_planner::{
    contraction::contrator::Contractor, dijkstra::ch_dijkstra::ChDijsktra,
    fmi_reader::GraphFileReader, graph::bidirectional_graph::BidirectionalGraph,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let graph_file_reader = GraphFileReader::new();
    let graph = graph_file_reader.from_file(args[1].as_str());
    let graph = BidirectionalGraph::from_graph(&graph);

    let number_nodes = graph.outgoing_edges.len();
    let before = Instant::now();
    let mut contractor = Contractor::new(graph);

    let shortcuts = contractor.contract(Duration::from_secs_f32(24.0 * 60.0 * 60.0));
    println!("there are {} shortcuts", shortcuts.len());
    let graph = contractor.get_graph().unwrap();

    let file_name = args[1].clone() + ".graph.json";
    let mut writer = BufWriter::new(File::create(file_name.as_str()).unwrap());
    serde_json::to_writer(&mut writer, &graph).unwrap();
    writer.flush().unwrap();

    println!(
        "contracting graph took {:?}, there are {} shortcuts",
        before.elapsed(),
        shortcuts.len()
    );

    let mut times = Vec::new();
    let dijskstra = ChDijsktra::new(graph);

    println!("starting route timing");
    let mut rng = rand::thread_rng();
    for _ in (0..1_000).progress() {
        let source = rng.gen_range(0..number_nodes) as u32;
        let target = rng.gen_range(0..number_nodes) as u32;
        let before = Instant::now();
        let _ = dijskstra.single_pair_shortest_path(source, target);
        times.push(before.elapsed());
    }

    println!("sum of time is {:?}", times.iter().sum::<Duration>());
    println!(
        "average time was {:?}",
        times.iter().sum::<Duration>() / times.len() as u32
    );
}
