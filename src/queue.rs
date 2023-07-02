pub struct BucketQueue {
    buckets: Vec<Vec<usize>>,
    current_index: usize,
}

impl BucketQueue {
    pub fn new(max: usize) -> BucketQueue {
        BucketQueue {
            buckets: vec![Vec::new(); max + 1],
            current_index: 0,
        }
    }

    pub fn push(&mut self, cost: usize, value: usize) {
        let index = cost % self.buckets.len();
        self.buckets[index].push(value);
    }

    pub fn pop(&mut self) -> Option<usize> {
        for i in self.current_index..self.current_index + self.buckets.len() {
            let index = i % self.buckets.len();
            let node_option = self.buckets[index].pop();
            if node_option.is_some() {
                self.current_index = i % self.buckets.len();
                return node_option;
            }
        }

        None
    }
}
