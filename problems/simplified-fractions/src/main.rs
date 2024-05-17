fn main() {
    assert_eq!(Solution::simplified_fractions(2), vec!["1/2"]);
    assert_eq!(Solution::simplified_fractions(3), vec!["1/2", "1/3", "2/3"]);
    assert_eq!(
        Solution::simplified_fractions(4),
        vec!["1/2", "1/3", "2/3", "1/4", "3/4"]
    );
}

struct Solution;
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Self::gcd(b, a % b);
    }
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        if n == 1 {
            return vec![];
        }
        let mut res = vec![];
        for i in 2..=n {
            for j in 1..i {
                if Self::gcd(i, j) == 1 {
                    res.push(format!("{}/{}", j, i));
                }
            }
        }
        return res;
    }
}
