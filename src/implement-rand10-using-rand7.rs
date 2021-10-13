fn main() {
    println!("debug {}", Solution::rand10());
}

pub struct Solution {}
use rand::{thread_rng, Rng};
fn rand7() -> i32 {
    return thread_rng().gen_range(1..=7);
}
/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        return 1 + (1..=10).map(|_x| rand7()).fold(0, |sum, a| sum + a) % 10;
    }
}
