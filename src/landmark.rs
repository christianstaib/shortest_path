use std::{cmp::max, collections::HashMap};

use ahash::RandomState;

#[derive(Clone)]
pub struct Landmark {
    landmark: u32,
    cost_to: HashMap<u32, u32, RandomState>,
    cost_from: HashMap<u32, u32, RandomState>,
}

impl Landmark {
    pub fn new(
        landmark: u32,
        cost_to: HashMap<u32, u32, RandomState>,
        cost_from: HashMap<u32, u32, RandomState>,
    ) -> Self {
        Landmark {
            landmark,
            cost_to,
            cost_from,
        }
    }

    pub fn is_reachable(&self, source: u32, target: u32) -> bool {
        self.cost_from.get(&source).is_some() & self.cost_to.get(&target).is_some()
    }

    pub fn landmark(self) -> u32 {
        self.landmark
    }

    pub fn upper_bound(&self, source: u32, target: u32) -> Option<u32> {
        let cost_from_source = self.cost_from.get(&source)?;
        let cost_to_target = self.cost_to.get(&target)?;

        Some(cost_from_source + cost_to_target)
    }

    pub fn lower_bound(&self, source: u32, target: u32) -> Option<u32> {
        let cost_to_source = self.cost_to.get(&source)?;
        let cost_to_target = self.cost_to.get(&target)?;

        let cost_from_source = self.cost_from.get(&source)?;
        let cost_from_target = self.cost_from.get(&target)?;

        Some(max(
            cost_to_target.checked_sub(*cost_to_source).unwrap_or(0),
            cost_from_source.checked_sub(*cost_from_target).unwrap_or(0),
        ))
    }
}
