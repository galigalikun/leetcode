fn main() {
    assert_eq!(Solution::judge_circle(String::from("UD")), true);
    assert_eq!(Solution::judge_circle(String::from("LL")), false);
}

struct Solution {}
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut p = (0, 0);
        for m in moves.chars() {
            match m {
                'U' => p.1 += 1,
                'D' => p.1 -= 1,
                'L' => p.0 -= 1,
                'R' => p.0 += 1,
                _ => {}
            }
        }
        return if p == (0, 0) { true } else { false };
    }
}
