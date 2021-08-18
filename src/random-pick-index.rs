use rand::{prelude::SliceRandom, thread_rng, Rng};
struct Solution {
    nums: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution {
            nums: nums,
            rng: thread_rng(),
        }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let mut result = vec![];
        let mut idx = 0;
        for n in &self.nums {
            if n == &target {
                result.push(idx);
            }
            idx += 1;
        }
        result.shuffle(&mut self.rng);
        return result[0];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
fn main() {
    let mut obj = Solution::new(vec![1, 2, 3, 3, 3]);
    let ret_1: i32 = obj.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
}
