pub struct Segment {
    pub id: usize,
    pub start: usize,
    pub end: usize,
}

pub struct SegmentTree {
    tree: Vec<Vec<usize>>,
    max_val: usize,
    segments: Vec<Segment>,
}

impl SegmentTree {
    pub fn new(max_val: usize) -> Self {
        SegmentTree {
            tree: vec![Vec::new(); 4 * (max_val + 1)],
            max_val,
            segments: Vec::new(),
        }
    }

    pub fn insert(&mut self, start: usize, end: usize) {
        let id = segments.len();
        let segment = Segment { id, start, end };
        self.segments.push(segment);
        self.insert_rec(1, 0, self.max_val, start, end, id);
    }

    fn insert_rec(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        seg_start: usize,
        seg_end: usize,
        id: usize,
    ) {
        if seg_start <= start && seg_end >= end {
            self.tree[node].push(id);
            return;
        }

        let mid = start + (end - start) / 2;
        let left_node = 2 * node;
        let right_node = 2 * node + 1;

        if seg_start <= mid {
            self.insert_rec(left_node, start, mid, seg_start, seg_end, id);
        }

        if seg_end > mid {
            self.insert_rec(right_node, mid + 1, end, seg_start, seg_end, id);
        }
    }

    pub fn query(&self, x: usize) -> Vec<Segment> {
        if x > self.max_val {
            return Vec::new();
        }

        let mut result_ids: Vec<usize> = Vec::new();
        self.query_rec(1, 0, self.max_val, x, &mut result_ids);

        result_ids.into_iter().map(|id| self.segments[id]).collect()
    }

    fn query_rec(&self, node: usize, start: usize, end: usize, x: usize, result: &mut Vec<usize>) {
        result.extend_from_slice(&self.tree[node]);

        if start == end {
            return;
        }

        let mid = start + (end - start) / 2;
        let left_node = 2 * node;
        let right_node = 2 * node + 1;

        if x <= mid {
            self.query_rec(left_node, start, mid, x, result);
        } else {
            self.query_rec(right_node, mid+1, end, x, result);
        }

    }
}
