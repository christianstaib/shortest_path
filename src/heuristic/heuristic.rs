pub trait Heuristic {
    fn is_reachable(&self, source: u32, target: u32) -> Option<bool>;
    fn upper_bound(&self, source: u32, target: u32) -> Option<u32>;
    fn lower_bound(&self, source: u32, target: u32) -> Option<u32>;
}
