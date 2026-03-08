fn main() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec!["a", "b", "c", "c", "e", "d"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["b", "c", "b", "e", "b", "e"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec![2, 5, 5, 1, 2, 20]
        ),
        28
    );
    assert_eq!(
        Solution::minimum_cost(
            "abcdefgh".to_string(),
            "acdeeghh".to_string(),
            vec!["bcd", "fgh", "thh"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["cde", "thh", "ghh"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec![1, 3, 5]
        ),
        9
    );
    assert_eq!(
        Solution::minimum_cost(
            "abcdefgh".to_string(),
            "addddddd".to_string(),
            vec!["bcd", "defgh"].iter().map(|s| s.to_string()).collect(),
            vec!["cde", "thh", "ghh"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec![100, 1578]
        ),
        -1
    );
}

struct Solution;
impl Solution {
    fn izip<T, U, V>(a: Vec<T>, b: Vec<U>, c: Vec<V>) -> impl Iterator<Item = (T, U, V)> {
        a.into_iter()
            .zip(b.into_iter())
            .zip(c.into_iter())
            .map(|((x, y), z)| (x, y, z))
    }
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut ans = 0;
        for (o, c, cost) in Self::izip(original, changed, cost) {
            if o == source && c == target {
                ans += cost as i64;
            }
        }
        ans
    }
}
