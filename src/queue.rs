pub struct BucketQueue {
    buckets: Vec<Vec<usize>>,
    mod_number: usize,
    current_index: usize,
}

impl BucketQueue {
    pub fn new(min_max_difference: usize) -> BucketQueue {
        BucketQueue {
            buckets: vec![Vec::new(); min_max_difference + 1],
            mod_number: min_max_difference + 1,
            current_index: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.iter().all(|bucket| bucket.is_empty())
    }

    pub fn push(&mut self, cost: usize, value: usize) {
        self.buckets[cost % self.mod_number].push(value);
    }

    pub fn pop(&mut self) -> Option<usize> {
        for i in self.current_index..self.current_index + self.mod_number {
            let node_option = self.buckets[i % self.mod_number].pop();
            if node_option.is_some() {
                self.current_index = i % self.mod_number;
                return node_option;
            }
        }

        None
    }
}
