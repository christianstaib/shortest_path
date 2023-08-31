use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub key: u32,
    pub value: u32,
}

impl State {
    pub fn new(key: u32, value: u32) -> Self {
        Self { key, value }
    }
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
            .key
            .cmp(&self.key)
            .then_with(|| self.value.cmp(&other.value))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
