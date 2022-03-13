fn main() {
    assert_eq!(Solution::frequency_sort("tree".to_string()), "eert");
    assert_eq!(Solution::frequency_sort("cccaaa".to_string()), "aaaccc");
    assert_eq!(Solution::frequency_sort("Aabb".to_string()), "bbAa");
}

struct Solution {}
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = vec![('\0', 0); 127];
        for c in s.chars() {
            let (_, n) = map[c as usize];
            map[c as usize] = (c, n + 1);
        }
        map.sort_by(|(_, a), (_, b)| b.cmp(a));

        return map
            .iter()
            .filter(|(_, x)| x > &0)
            .map(|(c, n)| std::iter::repeat(c).take(*n).collect::<String>())
            .collect::<String>();
    }
}
