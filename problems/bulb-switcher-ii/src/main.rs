fn main() {
    assert_eq!(Solution::flip_lights(1, 1), 2);
    assert_eq!(Solution::flip_lights(2, 1), 3);
    assert_eq!(Solution::flip_lights(3, 1), 4);
    assert_eq!(Solution::flip_lights(1, 2), 2);
}

// https://baihuqian.github.io/2018-07-31-bulb-switcher-ii/
struct Solution {}
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let key = (std::cmp::min(n, 3) - 1) as usize;
        return match presses {
            0 => 1,
            1 => vec![2, 3, 4][key],
            2 => vec![2, 4, 7][key],
            _ => vec![2, 4, 8][key],
        };
    }
}
