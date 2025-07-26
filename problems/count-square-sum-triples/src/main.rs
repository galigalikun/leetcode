fn main() {
    assert_eq!(Solution::count_triples(5), 2);
    assert_eq!(Solution::count_triples(10), 4);
}

struct Solution;
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for a in 1..=n {
            for b in a..=n {
                let c2 = a * a + b * b;
                let c = (c2 as f64).sqrt() as i32;
                if c * c == c2 && c <= n {
                    count += 1;
                }
            }
        }
        count
    }
}
