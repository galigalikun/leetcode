fn main() {
    assert_eq!(Solution::num_water_bottles(9, 3), 13);
    assert_eq!(Solution::num_water_bottles(15, 4), 19);
}

struct Solution;
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut empty_bottles = 0;
        let mut total = 0;
        let mut full_bottles = num_bottles;
        while full_bottles > 0 {
            total += full_bottles;
            empty_bottles += full_bottles;
            full_bottles = empty_bottles / num_exchange;
            empty_bottles = empty_bottles % num_exchange;
        }
        return total;
    }
}
