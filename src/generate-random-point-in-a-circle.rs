use rand::{thread_rng, Rng};

struct Solution {
    rng: rand::rngs::ThreadRng,
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            rng: thread_rng(),
            radius: radius,
            x_center: x_center,
            y_center: y_center,
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let r = self.rng.gen_range(0f64..self.radius);
        let a = self.rng.gen_range(0f64..2f64 * std::f64::consts::PI);
        println!("r a {} {}", r, a);
        return vec![r * a.cos() + self.x_center, r * a.sin() + self.y_center];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
fn main() {
    let mut obj = Solution::new(1.0, 0.0, 0.0);
    let ret_1: Vec<f64> = obj.rand_point();
    println!("debug {:?}", ret_1);

    let mut obj_2 = Solution::new(0.01, -73839.1, -3289891.3);
    let ret_2: Vec<f64> = obj_2.rand_point();
    println!("debug {:?}", ret_2);
}
