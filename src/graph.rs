use indicatif::ProgressBar;
use std::fs::File;
use std::io::{self, BufRead};
const SKIP_LINES: usize = 5;

#[derive(Clone)]
pub struct Edge {
    pub source_id: usize,
    pub target_id: usize,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    //pub id2: usize,
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub edges_start_at: Vec<usize>,
}

pub fn _min_cost_per_unit(graph: &Graph) -> f32 {
    graph
        .edges
        .iter()
        .filter(|edge| (edge.cost != 0) & (edge.source_id != edge.target_id))
        .map(|edge| {
            let from_node = &graph.nodes[edge.source_id];
            let to_node = &graph.nodes[edge.target_id];
            let distance = _distance(from_node, to_node);
            edge.cost as f32 / distance
        })
        .min_by(|x, y| x.total_cmp(y))
        .unwrap()
}

impl Graph {
    pub fn from_file(filename: &str) -> Graph {
        let nodes_and_edges = Graph::get_nodes_and_edges(filename);

        let nodes = nodes_and_edges.0;
        let mut edges = nodes_and_edges.1;
        let number_of_nodes = nodes.len();

        let edges_start_for_node: Vec<usize> =
            get_edges_start_for_node(&mut edges, number_of_nodes);

        let graph = Graph {
            nodes: nodes.clone(),
            edges: edges.clone(),
            edges_start_at: edges_start_for_node.clone(),
        };

        graph
    }

    fn get_nodes_and_edges(filename: &str) -> (Vec<Node>, Vec<Edge>) {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let mut i = 0;
        let bar = ProgressBar::new((number_of_nodes + number_of_edges) as u64);
        let nodes: Vec<Node> = lines
            .by_ref()
            .take(number_of_nodes)
            .map(|node_line| {
                i += 1;
                if i % 10_000 == 0 {
                    bar.inc(10_000);
                }

                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let id: usize = values.next().unwrap().parse().unwrap();
                let _id2: usize = values.next().unwrap().parse().unwrap();
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
                let _elevation: f32 = values.next().unwrap().parse().unwrap();

                Node {
                    id,
                    latitude,
                    longitude,
                }
            })
            .collect();

        let edges: Vec<Edge> = lines
            .by_ref()
            .take(number_of_edges)
            .map(|edge_line| {
                i += 1;
                if i % 10_000 == 0 {
                    bar.inc(10_000);
                }

                let line = edge_line.unwrap();
                let mut values = line.split_whitespace();
                let source_id: usize = values.next().unwrap().parse().unwrap();
                let target_id: usize = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                let _type: u32 = values.next().unwrap().parse().unwrap();
                let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                Edge {
                    source_id,
                    target_id,
                    cost,
                }
            })
            .collect();

        bar.finish();

        (nodes, edges)
    }

    pub fn clone_and_invert(&self) -> Graph {
        let nodes = self.nodes.clone();
        let mut edges = self
            .edges
            .iter()
            .map(|edge| Edge {
                source_id: edge.target_id,
                target_id: edge.source_id,
                cost: edge.cost,
            })
            .collect();

        let edges_start_for_node: Vec<usize> = get_edges_start_for_node(&mut edges, nodes.len());

        Graph {
            nodes: nodes.clone(),
            edges: edges.clone(),
            edges_start_at: edges_start_for_node.clone(),
        }
    }
}

pub fn get_edges_start_for_node(edges: &mut Vec<Edge>, number_of_nodes: usize) -> Vec<usize> {
    let mut edges_start_for_node: Vec<usize> = vec![0; number_of_nodes + 1];

    // temporarrly adding a node in order to generate the list
    edges.push(Edge {
        source_id: number_of_nodes,
        target_id: 0,
        cost: 0,
    });
    edges.sort_unstable_by_key(|edge| edge.source_id);

    let mut current = 0;
    for (i, edge) in edges.iter().enumerate() {
        if edge.source_id != current {
            for index in (current + 1)..=edge.source_id {
                edges_start_for_node[index] = i;
            }
            current = edge.source_id;
        }
    }
    edges.pop();

    edges_start_for_node
}

pub fn _distance(from: &Node, to: &Node) -> f32 {
    ((from.latitude - to.latitude).powi(2) + (from.longitude - to.longitude).powi(2)).sqrt()
}
