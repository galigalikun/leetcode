fn main() {
    assert_eq!(
        Solution::rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]),
        6,
    );
    assert_eq!(
        Solution::rectangle_area(vec![vec![0, 0, 1_000_000_000, 1_000_000_000]]),
        49,
    );
}

struct Solution;

#[derive(Clone, Copy)]
struct Event {
    x: i64,
    y1: i64,
    y2: i64,
    delta: i32,
}

struct SegmentTree {
    ys: Vec<i64>,
    cover_count: Vec<i32>,
    covered_len: Vec<i64>,
}

impl SegmentTree {
    fn new(ys: Vec<i64>) -> Self {
        let n = ys.len().saturating_mul(4);
        Self {
            ys,
            cover_count: vec![0; n],
            covered_len: vec![0; n],
        }
    }

    fn update(&mut self, node: usize, seg_l: usize, seg_r: usize, ql: usize, qr: usize, delta: i32) {
        if qr < seg_l || seg_r < ql {
            return;
        }

        if ql <= seg_l && seg_r <= qr {
            self.cover_count[node] += delta;
            self.pull(node, seg_l, seg_r);
            return;
        }

        let mid = (seg_l + seg_r) / 2;
        self.update(node * 2, seg_l, mid, ql, qr, delta);
        self.update(node * 2 + 1, mid + 1, seg_r, ql, qr, delta);
        self.pull(node, seg_l, seg_r);
    }

    fn pull(&mut self, node: usize, seg_l: usize, seg_r: usize) {
        if self.cover_count[node] > 0 {
            self.covered_len[node] = self.ys[seg_r + 1] - self.ys[seg_l];
            return;
        }

        if seg_l == seg_r {
            self.covered_len[node] = 0;
            return;
        }

        self.covered_len[node] = self.covered_len[node * 2] + self.covered_len[node * 2 + 1];
    }

    fn total_covered_len(&self) -> i64 {
        self.covered_len[1]
    }
}

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut ys: Vec<i64> = Vec::with_capacity(rectangles.len() * 2);
        let mut events: Vec<Event> = Vec::with_capacity(rectangles.len() * 2);

        for rect in rectangles {
            let x1 = rect[0] as i64;
            let y1 = rect[1] as i64;
            let x2 = rect[2] as i64;
            let y2 = rect[3] as i64;

            ys.push(y1);
            ys.push(y2);
            events.push(Event {
                x: x1,
                y1,
                y2,
                delta: 1,
            });
            events.push(Event {
                x: x2,
                y1,
                y2,
                delta: -1,
            });
        }

        ys.sort_unstable();
        ys.dedup();

        if ys.len() < 2 || events.is_empty() {
            return 0;
        }

        events.sort_unstable_by_key(|e| e.x);
        let mut seg_tree = SegmentTree::new(ys.clone());

        let mut area: i64 = 0;
        let mut i = 0usize;
        let mut prev_x = events[0].x;
        let last_seg = ys.len() - 2;

        while i < events.len() {
            let x = events[i].x;
            let dx = x - prev_x;
            let covered_y = seg_tree.total_covered_len();
            area = (area + covered_y * dx) % MOD;

            while i < events.len() && events[i].x == x {
                let y1_idx = ys.binary_search(&events[i].y1).unwrap_or(0);
                let y2_idx = ys.binary_search(&events[i].y2).unwrap_or(0);

                if y1_idx < y2_idx {
                    seg_tree.update(1, 0, last_seg, y1_idx, y2_idx - 1, events[i].delta);
                }

                i += 1;
            }

            prev_x = x;
        }

        (area % MOD) as i32
    }
}
