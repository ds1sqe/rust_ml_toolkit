#[derive(Clone)]
pub struct Cost {
    pub idx: usize,
    pub val: f64,
}
#[derive(Clone)]
pub struct CostInfo {
    pub total: f64,
    max_idx: usize,
    min_idx: usize,
    pub avg: f64,
    pub costs: Vec<Cost>,
}

impl CostInfo {
    pub fn new() -> Self {
        CostInfo {
            total: 0_f64,
            avg: 0_f64,
            max_idx: 0,
            min_idx: 0,
            costs: Vec::new(),
        }
    }

    pub fn max(&self) -> &Cost {
        &self.costs[self.max_idx]
    }
    pub fn min(&self) -> &Cost {
        &self.costs[self.min_idx]
    }
    pub fn avg(&self) -> &f64 {
        &self.avg
    }

    pub fn push(&mut self, cost: f64) {
        let old_len = self.costs.len();

        self.costs.push(Cost {
            idx: old_len,
            val: cost,
        });

        self.total += cost;

        self.avg = self.total / (old_len + 1) as f64;

        let cur_idx = old_len;

        if self.max().val < cost {
            self.max_idx = cur_idx;
        }
        if self.min().val > cost {
            self.min_idx = cur_idx;
        }
    }
}
