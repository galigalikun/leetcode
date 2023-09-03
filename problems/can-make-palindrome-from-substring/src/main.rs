fn main() {
    assert_eq!(
        Solution::can_make_pali_queries(
            "abcda".to_string(),
            vec![
                vec![3, 3, 0],
                vec![1, 2, 0],
                vec![0, 3, 1],
                vec![0, 3, 2],
                vec![0, 4, 1]
            ]
        ),
        vec![true, false, false, true, true]
    );
    assert_eq!(
        Solution::can_make_pali_queries("lyb".to_string(), vec![vec![0, 1, 0], vec![2, 2, 1]]),
        vec![false, true]
    );
}

struct Solution;
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut result = vec![];
        for query in queries {
            let mut count = 0;
            let mut map = vec![0; 26];
            for i in query[0]..=query[1] {
                let index = (s.as_bytes()[i as usize] - 97) as usize;
                map[index] += 1;
            }
            for i in 0..26 {
                if map[i] % 2 == 1 {
                    count += 1;
                }
            }
            if count / 2 <= query[2] {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        return result;
    }
}
