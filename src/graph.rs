use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
pub struct Edge {
    pub source_id: u32,
    pub target_id: u32,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub id: u32,
    pub longitude: f32,
    pub latitude: f32,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub edges_start_at: Vec<u32>,
}

impl Graph {
    pub fn from_file(filename: &str) -> Graph {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines();
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let nodes: Vec<Node> = lines
            .by_ref()
            .take(number_of_nodes)
            .map(|node_line| {
                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let node_id: u32 = values.next().unwrap().parse().unwrap();
                //let _node_id2: usize = values.next().unwrap().parse().unwrap();
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
                // let _elevation: f32 = values.next().unwrap().parse().unwrap();

                Node {
                    id: node_id,
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
                let source_id: u32 = values.next().unwrap().parse().unwrap();
                let target_id: u32 = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                // let _type: u32 = values.next().unwrap().parse().unwrap();
                // let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                Edge {
                    source_id,
                    target_id,
                    cost,
                }
            })
            .collect();

        let mut edges_start_for_node: Vec<u32> = vec![0; number_of_nodes + 1];

        // temporarrly adding a node in order to generate the list
        edges.push(Edge {
            source_id: number_of_nodes as u32,
            target_id: 0,
            cost: 0,
        });
        edges.sort_unstable_by_key(|edge| edge.source_id);

        let mut current = 0;
        for (i, edge) in edges.iter().enumerate() {
            if edge.source_id != current {
                for index in (current + 1)..=edge.source_id {
                    edges_start_for_node[index as usize] = i as u32;
                }
                current = edge.source_id;
            }
        }
        edges.pop();

        Graph {
            nodes: nodes.clone(),
            edges: edges.clone(),
            edges_start_at: edges_start_for_node.clone(),
        }
    }
}

pub struct Route {
    pub start: u32,
    pub end: u32,
    pub cost: u32,
    pub edges: Vec<Edge>,
}

pub fn get_route(
    graph: &Graph,
    start: u32,
    end: u32,
    used_edges: Vec<Option<u32>>,
) -> Option<Route> {
    let mut edges: Vec<Edge> = Vec::new();
    let mut current: u32 = end;

    while let Some(edge_index) = used_edges[current as usize] {
        current = graph.edges[edge_index as usize].source_id;
        edges.push(graph.edges[edge_index as usize].clone());
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
