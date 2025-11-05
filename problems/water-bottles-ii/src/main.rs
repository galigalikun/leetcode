fn main() {
    assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
    assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
}

struct Solution;
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drunk = num_bottles;
        let mut empty_bottles = num_bottles;

        while empty_bottles >= num_exchange {
            let new_bottles = empty_bottles / num_exchange;
            total_drunk += new_bottles;
            empty_bottles = empty_bottles % num_exchange + new_bottles;
        }

        total_drunk
    }
}
