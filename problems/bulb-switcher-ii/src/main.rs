fn main() {
    assert_eq!(Solution::flip_lights(1, 1), 2);
    assert_eq!(Solution::flip_lights(2, 1), 3);
    assert_eq!(Solution::flip_lights(3, 1), 4);
    assert_eq!(Solution::flip_lights(1, 2), 2);
}

struct Solution {}
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        return match n {
            1 | 2 => presses + n,
            _ => {
                let mut res = n;
                for i in 1..=presses {
                    res += i;
                }
                res
            }
        };
    }
}
