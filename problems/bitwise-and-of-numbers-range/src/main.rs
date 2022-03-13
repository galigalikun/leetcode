fn main() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);

    assert_eq!(Solution::range_bitwise_and(0, 1), 0);

    assert_eq!(Solution::range_bitwise_and(4, 8), 0);
}

struct Solution {}
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result: Option<i32> = None;
        for n in m..=n {
            match result.as_mut() {
                Some(v) => *v &= n,
                None => result = Some(n),
            }
        }
        if let Some(r) = result {
            return r;
        }
        return 0;
    }
}
