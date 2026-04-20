fn main() {
    assert_eq!(Solution::mirror_distance(25), 27);
    assert_eq!(Solution::mirror_distance(10), 9);
    assert_eq!(Solution::mirror_distance(7), 0);
}

struct Solution;
impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let r = n
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        return (n - r).abs();
    }
}
