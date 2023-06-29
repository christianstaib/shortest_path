use crate::dijkstra::*;
use rand::Rng;
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
    pub longitude: f32,
    pub latitude: f32,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub edges_start_at: Vec<usize>,
}

impl Graph {
    pub fn from_file(filename: &str) -> Graph {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let nodes: Vec<Node> = lines
            .by_ref()
            .take(number_of_nodes)
            .map(|node_line| {
                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let _node_id: usize = values.next().unwrap().parse().unwrap();
                let _node_id2: usize = values.next().unwrap().parse().unwrap();
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
                let _elevation: f32 = values.next().unwrap().parse().unwrap();

                Node {
                    latitude,
                    longitude,
                }
            })
            .collect();

        let mut edges: Vec<Edge> = lines
            .by_ref()
            .take(number_of_edges)
            .map(|edge_line| {
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

        let graph = Graph {
            nodes: nodes.clone(),
            edges: edges.clone(),
            edges_start_at: edges_start_for_node.clone(),
        };

        let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
        let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
        for edge in &graph.edges {
            outgoing[edge.source_id].push(edge.target_id);
            incoming[edge.target_id].push(edge.source_id);
        }

        let intersections: Vec<usize> = (0..graph.nodes.len())
            .into_iter()
            .filter(|&node_id| {
                let mut l1 = outgoing[node_id].clone();
                let mut l2 = incoming[node_id].clone();
                l1.sort();
                l2.sort();
                //(l1 != l2) | ((l1 == l2) & (l1.len() > 2));
                (outgoing[node_id].len() > 2) | ((outgoing[node_id].len() == 2) & (l1 != l2))
            })
            .collect();

        let h_factor = get_h_factor(&graph).unwrap() as f32;
        let mut extra_edges: Vec<Edge> = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("calculation extra edge {i}");
            let source_id = rng.gen_range(0..intersections.len());
            let target_id = rng.gen_range(0..intersections.len());

            let used_edges = a_star(&graph, source_id, target_id, h_factor);

            let route = get_route(&graph, source_id, target_id, used_edges);
            if route.is_some() {
                let cost = route.unwrap().cost;
                extra_edges.push(Edge {
                    source_id,
                    target_id,
                    cost,
                })
            }
        }

        edges.append(&mut extra_edges);
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

        let graph = Graph {
            nodes,
            edges,
            edges_start_at: edges_start_for_node,
        };
        graph
    }
}

pub struct Route {
    pub start: usize,
    pub end: usize,
    pub cost: u32,
    pub edges: Vec<Edge>,
}

pub fn get_route(
    graph: &Graph,
    start: usize,
    end: usize,
    used_edges: Vec<Option<usize>>,
) -> Option<Route> {
    let mut edges: Vec<Edge> = Vec::new();
    let mut current: usize = end;

    while let Some(edge_index) = used_edges[current] {
        current = graph.edges[edge_index].source_id;
        edges.push(graph.edges[edge_index].clone());
        if current == start {
            break;
        }
    }

    if current != start {
        return None;
    }

    Some(Route {
        start,
        end,
        cost: edges.iter().map(|edge| edge.cost).sum(),
        edges,
    })
}
