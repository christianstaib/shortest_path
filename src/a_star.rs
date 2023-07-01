use crate::dijkstra::*;
use crate::graph::*;
use crate::queue::*;
//use rand::Rng;
use std::collections::BinaryHeap;
use std::thread;
use std::time::Duration;

pub struct AStar {
    graph: Graph,
    cost_from: Vec<Vec<u32>>,
    cost_to: Vec<Vec<u32>>,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let graph_copy = Graph {
            nodes: graph.nodes.clone(),
            edges: graph.edges.clone(),
            edges_start_at: graph.edges_start_at.clone(),
        };
        let inverted_graph = graph_copy.invert();

        let max_edge_cost = graph_copy.edges.iter().map(|edge| edge.cost).max().unwrap();
        let dijkstra = Dijkstra {
            graph: graph_copy,
            max_edge_cost: max_edge_cost as usize,
        };

        let max_inverted_edge_cost = inverted_graph
            .edges
            .iter()
            .map(|edge| edge.cost)
            .max()
            .unwrap();
        let inverted_dijkstra = Dijkstra {
            graph: inverted_graph,
            max_edge_cost: max_inverted_edge_cost as usize,
        };

        //let mut rng = rand::thread_rng();
        let mut cost_from: Vec<Vec<u32>> = Vec::new();
        let mut cost_to: Vec<Vec<u32>> = Vec::new();
        for landmark in [12] {
            //, 234, 3456, 45678, 567890] {
            println!("calculating landmark no {}", landmark);
            //let landmark = rng.gen_range(0..dijkstra.graph.nodes.len());
            cost_from.push(dijkstra.get_costs_from_source(landmark));
            cost_to.push(inverted_dijkstra.get_costs_from_source(landmark));
        }

        AStar {
            graph,
            cost_from,
            cost_to,
        }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self.a_star(source_id, target_id);
        let route = get_route(&self.graph, source_id, target_id, used_edges);
        route
    }

    pub fn a_star(&self, from_node_id: usize, to_node_id: usize) -> Vec<Option<usize>> {
        let dijkstra = Dijkstra::_new(self.graph.invert());
        let true_cost = dijkstra.get_costs_from_source(to_node_id);
        let mut is_admissible = true;

        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        queue.push(State {
            node_cost: 0,
            node_id: from_node_id,
        });

        let mut edge_from_predecessor: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        while !queue.is_empty() {
            let state = queue.pop().unwrap();
            if is_expanded[state.node_id] {
                continue;
            }
            if state.node_id == to_node_id {
                break;
            }
            is_expanded[state.node_id] = true;

            for edge_id in self.graph.edges_start_at[state.node_id]
                ..self.graph.edges_start_at[state.node_id + 1]
            {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = node_cost[state.node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = Some(edge_id);
                    node_cost[edge.target_id] = alternative_cost;

                    let mut h_value = 0;
                    for i in 0..self.cost_from.len() {
                        let cost_from = &self.cost_from[i];
                        if cost_from[edge.target_id] > cost_from[to_node_id] {
                            h_value = std::cmp::max(
                                h_value,
                                cost_from[edge.target_id] - cost_from[to_node_id],
                            );
                        }
                        let cost_to = &self.cost_to[i];
                        if cost_to[to_node_id] > cost_to[edge.target_id] {
                            h_value = std::cmp::max(
                                h_value,
                                cost_to[to_node_id] - cost_to[edge.target_id],
                            )
                        }
                        if h_value >= 1 {
                            h_value -= 1;
                        }
                    }

                    if h_value > true_cost[edge.target_id] {
                        is_admissible = false;
                    }

                    queue.push(State {
                        node_cost: alternative_cost + true_cost[edge.target_id],
                        node_id: edge.target_id,
                    });
                }
            }
        }

        println!("next line is corrcect? {}", is_admissible);
        edge_from_predecessor
    }
}

pub fn _get_min_max_nodes(graph: &Graph) -> (Node, Node) {
    let min_lat = graph
        .nodes
        .iter()
        .map(|node| node.latitude)
        .min_by(|x, y| x.total_cmp(y))
        .unwrap();
    let max_lat = graph
        .nodes
        .iter()
        .map(|node| node.latitude)
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();
    let min_long = graph
        .nodes
        .iter()
        .map(|node| node.longitude)
        .min_by(|x, y| x.total_cmp(y))
        .unwrap();
    let max_long = graph
        .nodes
        .iter()
        .map(|node| node.longitude)
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();
    (
        Node {
            id: 0,
            latitude: min_lat,
            longitude: min_long,
        },
        Node {
            id: 0,
            latitude: max_lat,
            longitude: max_long,
        },
    )
}

pub fn _get_h_factor(graph: &Graph) -> Option<u32> {
    let min_ratio = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let ratio = edge.cost as f32 / _distance(&source_node, &target_node);

            ratio
        })
        .filter(|x| x.is_normal())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();

    let is_admissible = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let h = min_ratio * _distance(&source_node, &target_node);
            h as u32 <= edge.cost
        })
        .all(|x| x == true);

    match is_admissible {
        true => Some(min_ratio as u32),
        false => None,
    }
}
