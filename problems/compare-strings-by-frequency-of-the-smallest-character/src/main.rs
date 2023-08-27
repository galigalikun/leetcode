fn main() {
    assert_eq!(
        Solution::num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
        vec![1]
    );
    assert_eq!(
        Solution::num_smaller_by_frequency(
            vec!["bbb".to_string(), "cc".to_string()],
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ]
        ),
        vec![1, 2]
    );
}

struct Solution;
impl Solution {
    fn f(s: String) -> i32 {
        let mut count = 0;
        let mut min = 'z';
        for c in s.chars() {
            if c < min {
                min = c;
                count = 1;
            } else if c == min {
                count += 1;
            }
        }
        return count;
    }
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut queries_count = vec![];
        let mut words_count = vec![];
        for query in queries {
            queries_count.push(Solution::f(query));
        }
        for word in words {
            words_count.push(Solution::f(word));
        }
        let mut result = vec![];
        for query_count in queries_count {
            let mut count = 0;
            for word_count in &words_count {
                if query_count < *word_count {
                    count += 1;
                }
            }
            result.push(count);
        }
        return result;
    }
}
