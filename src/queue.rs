use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub node_cost: u32,
    pub node_id: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .node_cost
            .cmp(&self.node_cost)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
