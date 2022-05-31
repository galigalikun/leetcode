use rand::Rng;

struct Solution {
    pub a: i32,
    pub b: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        Solution {
            a: n,
            b: blacklist.iter().sum(),
        }
    }

    fn pick(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(0..self.a);
        while self.b > 0 {
            if r < self.b {
                r += 1;
            } else {
                r -= self.b;
            }
            self.b -= 1;
        }
        return r;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
fn main() {
    let mut obj = Solution::new(7, vec![2, 3, 5]);
    assert_eq!(obj.pick(), 0);
    assert_eq!(obj.pick(), 4);
    assert_eq!(obj.pick(), 1);
    assert_eq!(obj.pick(), 6);
    assert_eq!(obj.pick(), 1);
    assert_eq!(obj.pick(), 0);
    assert_eq!(obj.pick(), 4);
}
