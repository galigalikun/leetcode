use rand::{thread_rng, Rng};

struct Solution {
    rng: rand::rngs::ThreadRng,
    rects: Vec<[i32; 4]>,
    prefix_points: Vec<i64>,
    total_points: i64,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut normalized_rects = Vec::with_capacity(rects.len());
        let mut prefix_points = Vec::with_capacity(rects.len());
        let mut running_total = 0_i64;

        for rect in rects {
            let a = rect[0];
            let b = rect[1];
            let x = rect[2];
            let y = rect[3];

            let width = i64::from(x - a + 1);
            let height = i64::from(y - b + 1);
            let points = width * height;

            running_total += points;
            normalized_rects.push([a, b, x, y]);
            prefix_points.push(running_total);
        }

        Solution {
            rng: thread_rng(),
            rects: normalized_rects,
            prefix_points,
            total_points: running_total,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let target = self.rng.gen_range(1..=self.total_points);
        let rect_idx = self
            .prefix_points
            .binary_search(&target)
            .unwrap_or_else(|idx| idx);

        let prev_prefix = if rect_idx == 0 {
            0
        } else {
            self.prefix_points[rect_idx - 1]
        };

        let offset = target - prev_prefix - 1;
        let [a, b, x, _y] = self.rects[rect_idx];
        let width = i64::from(x - a + 1);

        let dx = offset % width;
        let dy = offset / width;

        vec![a + dx as i32, b + dy as i32]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
fn main() {
    let mut obj = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    for _ in 0..100 {
        let p = obj.pick();
        let in_first = (-2..=1).contains(&p[0]) && (-2..=1).contains(&p[1]);
        let in_second = (2..=4).contains(&p[0]) && (2..=6).contains(&p[1]);
        assert!(in_first || in_second);
    }
}
