use rand::{thread_rng, Rng};
struct Solution {
    rng: rand::rngs::ThreadRng,
    rects: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(rects: Vec<Vec<i32>>) -> Self {
        Solution{
            rng: thread_rng(),
            rects: rects,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let r = self.rng.gen_range(0..self.rects[0][0]);
        return vec![r, r];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
fn main() {
    let mut obj = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    let ret_1: Vec<i32> = obj.pick();
    assert_eq!(ret_1, vec![]);
}
