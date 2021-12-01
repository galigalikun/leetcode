use rand::{thread_rng, Rng};
struct Solution {
    data: Vec<usize>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut data = vec![];
        for i in 0..w.len() {
            data.extend(vec![i; w[i] as usize]);
        }
        Solution {
            data: data,
            rng: thread_rng(),
        }
    }

    fn pick_index(&mut self) -> i32 {
        let i = self.rng.gen_range(0..self.data.len());
        return self.data[i] as i32;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
fn main() {
    let mut obj = Solution::new(vec![1]);
    assert_eq!(obj.pick_index(), 0);
}
