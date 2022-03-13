fn main() {
    assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
    assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
    assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
}

struct Solution {}
// https://just4once.gitbooks.io/leetcode-notes/content/leetcode/binary-search/475-heaters.html
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut heater = heaters;
        heater.sort();
        let mut r = 0;
        for h in houses {
            let mut i = -1;
            for j in 0..heater.len() {
                if heater[j] == h {
                    i = heater[j];
                    break;
                } else if heater[j] > h {
                    break;
                }
                i -= 1;
            }
            if i < 0 {
                let idx = (-(i + 1)) as usize;
                let left_r = if idx > 0 {
                    h - heater[idx - 1]
                } else {
                    std::i32::MAX
                };
                let right_r = if idx < heater.len() {
                    heater[idx] - h
                } else {
                    std::i32::MAX
                };

                r = std::cmp::max(r, std::cmp::min(left_r, right_r));
            }
        }
        return r;
    }
}
