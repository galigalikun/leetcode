use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Solution {
    rows: i32,
    cols: i32,
    remaining: i32,
    remap: HashMap<i32, i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            rows: m,
            cols: n,
            remaining: m * n,
            remap: HashMap::new(),
            rng: thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let idx = self.rng.gen_range(0..self.remaining);

        let actual = *self.remap.get(&idx).unwrap_or(&idx);

        let last = self.remaining - 1;
        let last_actual = *self.remap.get(&last).unwrap_or(&last);

        self.remap.insert(idx, last_actual);
        self.remap.remove(&last);
        self.remaining -= 1;

        vec![actual / self.cols, actual % self.cols]
    }

    fn reset(&mut self) {
        self.remaining = self.rows * self.cols;
        self.remap.clear();
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */
fn main() {
    let mut obj = Solution::new(3, 1);
    let mut seen = [false; 3];

    for _ in 0..3 {
        let p = obj.flip();
        assert_eq!(p[1], 0);
        assert!(p[0] >= 0 && p[0] < 3);
        seen[p[0] as usize] = true;
    }

    assert!(seen.iter().all(|v| *v));

    obj.reset();
    let p = obj.flip();
    assert_eq!(p[1], 0);
    assert!(p[0] >= 0 && p[0] < 3);
}
