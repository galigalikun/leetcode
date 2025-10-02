fn main() {
    assert_eq!(
        Solution::interchangeable_rectangles(vec![
            vec![4, 8],
            vec![3, 6],
            vec![10, 20],
            vec![15, 30]
        ]),
        6
    );
    assert_eq!(
        Solution::interchangeable_rectangles(vec![vec![4, 5], vec![7, 8]]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        let mut map = HashMap::new();
        for rect in rectangles {
            let gcd = gcd(rect[0], rect[1]);
            let ratio = (rect[0] / gcd, rect[1] / gcd);
            *map.entry(ratio).or_insert(0) += 1;
        }
        let mut count = 0;
        for &v in map.values() {
            if v > 1 {
                count += v * (v - 1) / 2;
            }
        }
        count as i64
    }
}
