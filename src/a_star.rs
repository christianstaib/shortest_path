use crate::dijkstra::*;
use crate::graph::*;
use crate::node_map::*;
use crate::route::*;
use rand::Rng;

pub struct AStar {
    graph: Graph,
    max_h_value: usize,
    cost_to: Vec<Vec<u32>>,
    cost_from: Vec<Vec<u32>>,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let graph_copy = Graph {
            nodes: graph.nodes.clone(),
            edges: graph.edges.clone(),
            edges_start_at: graph.edges_start_at.clone(),
        };

        let max_edge_cost = graph_copy.edges.iter().map(|edge| edge.cost).max().unwrap() as usize;
        let inverted_dijkstra = Dijkstra {
            graph: graph_copy.clone_and_invert(),
            max_edge_cost,
        };

        let dijkstra = Dijkstra {
            graph: graph_copy,
            max_edge_cost,
        };

        let mut rng = rand::thread_rng();
        let mut cost_to: Vec<Vec<u32>> = Vec::new();
        let mut cost_from: Vec<Vec<u32>> = Vec::new();
        let node_map = NodeMap::new(&graph, 4);
        for (i, square) in node_map.map.iter().flatten().enumerate() {
            println!(
                "calculating landmark {} of {} (choose one of {})",
                i + 1,
                node_map.map.len().pow(2),
                square.len()
            );
            if !square.is_empty() {
                let landmark = square[rng.gen_range(0..square.len())];
                cost_to.push(inverted_dijkstra.get_cost_from(landmark));
                cost_from.push(dijkstra.get_cost_from(landmark));
            }
        }

        let min_max_nodes = _get_min_max_nodes(&graph);
        let cost_per_unit = min_cost_per_unit(&graph);
        let max_h_value =
            3 * (cost_per_unit * _distance(&min_max_nodes.0, &min_max_nodes.1)) as usize;

        AStar {
            graph,
            max_h_value,
            cost_to,
            cost_from,
        }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self._get_used_edges(source_id, target_id);
        let route = get_route(&self.graph, source_id, target_id, used_edges);
        route
    }

    fn _get_used_edges(&self, from_id: usize, to_id: usize) -> Vec<Option<usize>> {
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); self.max_h_value * 2];
        let mut incoming_edge: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        buckets[0].push(from_id);
        let mut nodes_in_buckets = 1;
        'outer: for i in 0..self.max_h_value {
            while let Some(node_id) = buckets[i].pop() {
                nodes_in_buckets -= 1;
                if node_id == to_id {
                    break 'outer;
                }
                is_expanded[node_id] = true;

                let start_edge_id = self.graph.edges_start_at[node_id];
                let end_edge_id = self.graph.edges_start_at[node_id + 1];
                for edge_id in start_edge_id..end_edge_id {
                    let edge = &self.graph.edges[edge_id];
                    let alternative_cost = node_cost[node_id] + edge.cost;
                    if alternative_cost < node_cost[edge.target_id] {
                        incoming_edge[edge.target_id] = Some(edge_id);
                        node_cost[edge.target_id] = alternative_cost;

                        let h_value = self
                            .cost_to
                            .iter()
                            .zip(self.cost_from.iter())
                            .map(|(cost_to, cost_from)| {
                                (cost_to[edge.target_id] as i32 - cost_to[to_id] as i32)
                                    .max(cost_from[to_id] as i32 - cost_from[edge.target_id] as i32)
                            })
                            .max()
                            .unwrap();

                        //println!("h_value {}", h_value);

                        buckets[alternative_cost as usize + h_value as usize].push(edge.target_id);
                        nodes_in_buckets += 1
                    }
                }

                if nodes_in_buckets == 0 {
                    break 'outer;
                }
            }
        }

        incoming_edge
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
