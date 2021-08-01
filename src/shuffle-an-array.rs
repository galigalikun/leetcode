// https://goodgid.github.io/LeetCode-Shuffle-an-Array/
use rand::{thread_rng, Rng};
struct Solution {
    data: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution {
            data: nums,
            rng: thread_rng(),
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        return self.data.clone();
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut nums = self.data.clone();
        for j in 1..self.data.len() {
            let i:usize = self.rng.gen_range(0..j+1);
            if i == j {
                continue;
            }
            nums[i] ^= nums[j];
            nums[j] ^= nums[i];
            nums[i] ^= nums[j];
        }
        return nums;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
fn main() {
    let mut obj = Solution::new(vec![1, 2, 3]);
    let ret_1: Vec<i32> = obj.shuffle();
    assert_eq!(ret_1, vec![1, 3, 2]);
    let ret_2: Vec<i32> = obj.reset();
    assert_eq!(ret_2, vec![1, 2, 3]);
    let ret_3: Vec<i32> = obj.shuffle();
    assert_eq!(ret_3, vec![1, 2, 3]);
}
