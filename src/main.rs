use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

mod graph;
use crate::graph::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    node: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let graph = Graph::from_file("data/germany.fmi");
    println!("pathfinding start");

    let start_node: usize = rng.gen_range(0..graph.nodes.len());
    let end_node: usize = rng.gen_range(0..graph.nodes.len());

    let start = Instant::now();
    for _ in 0..10 {
        let predecessors = get_predcessors(&graph, start_node, end_node);
        if let Some(route) = predecessors {
            println!("found route");
        } else {
            println!("found no route");
        }
    }
    let end = start.elapsed();
    println!("Duration was {:.2?}", end / 10);
}

fn get_route(
    start_node: usize,
    end_node: usize,
    predecessors: Vec<Option<usize>>,
) -> Option<Vec<usize>> {
    let mut route: Vec<usize> = Vec::new();
    route.push(end_node);
    let mut current_node = end_node;

    while let Some(predecessor) = predecessors[current_node] {
        current_node = predecessor;
        route.push(predecessor);
    }

    if current_node == start_node {
        Some(route)
    } else {
        None
    }
}

fn get_predcessors(
    graph: &Graph,
    start_node: usize,
    end_node: usize,
) -> Option<Vec<Option<usize>>> {
    //if (start_node >= graph.nodes.len()) | (end_node >= graph.nodes.len()) {
    //    return None;
    //}

    let mut queue = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; graph.nodes.len()];
    let mut node_costs: Vec<u32> = vec![u32::MAX; graph.nodes.len()];
    let mut predecessors: Vec<Option<usize>> = vec![None; graph.nodes.len()];

    node_costs[start_node] = 0;
    queue.push(State {
        cost: 0,
        node: start_node,
    });

    loop {
        let state = queue.pop().unwrap();
        let current_node = state.node;
        if visited[current_node] == true {
            continue;
        }
        visited[current_node] = true;
        let current_cost = state.cost;

        for edge in &graph.edges[current_node] {
            if edge.target_id >= graph.nodes.len() {
                return None;
            }
            let new_cost = current_cost + edge.cost;
            if new_cost < node_costs[edge.target_id] {
                node_costs[edge.target_id] = new_cost;
                queue.push(State {
                    node: edge.target_id,
                    cost: new_cost,
                });
                predecessors[edge.target_id] = Some(current_node);
            }
        }

        if (current_node == end_node) | (queue.is_empty()) {
            break;
        }
    }

    Some(predecessors)
}
