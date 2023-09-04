pub trait PriorityTerm {
    fn priority(&self, v: u32) -> i32;
    fn update(&mut self, v: u32);
}
