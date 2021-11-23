use rand::{thread_rng, Rng};
struct Solution {
    matrix:Vec<Vec<i32>>,
    rng: rand::rngs::ThreadRng,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(m: i32, n: i32) -> Self {
        Solution{
            matrix:vec![vec![0;n as usize]; m as usize],
            rng:thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let i = self.rng.gen_range(0..self.matrix.len());
        let j = self.rng.gen_range(0..self.matrix[0].len());
        self.matrix[i][j] &= self.matrix[i][j];
        println!("debug {:?}", self.matrix);
        return vec![];
    }

    fn reset(&mut self) {
        self.matrix = vec![vec![0;self.matrix[0].len()]; self.matrix.len()];
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
    println!("{:?}", obj.flip());
    obj.reset();
    println!("{:?}", obj.flip());
}
