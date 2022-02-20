fn main() {
    assert_eq!(Solution::judge_square_sum(61), true);
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
    assert_eq!(Solution::judge_square_sum(2), true);
    assert_eq!(Solution::judge_square_sum(5487023), false);
    assert_eq!(Solution::judge_square_sum(10000000), true);
    assert_eq!(Solution::judge_square_sum(2147482647), true);
}

struct Solution {}
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let n = (c as f64).sqrt() as i64;
        for a in (0..=n).rev() {
            for b in 0..=n {
                let c2 = a.pow(2) + b.pow(2);
                if c2 == c as i64 {
                    return true;
                } else if c2 > c as i64 {
                    break;
                }
            }
        }

        return false;
    }
}
