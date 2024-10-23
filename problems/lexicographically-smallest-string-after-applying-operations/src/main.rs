fn main() {
    assert_eq!(Solution::find_lex_smallest_string("5525".to_string(), 9, 2), "2050");
    assert_eq!(Solution::find_lex_smallest_string("74".to_string(), 5, 1), "24");
    assert_eq!(Solution::find_lex_smallest_string("0011".to_string(), 4, 2), "0011");
}

struct Solution;
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut result = s.clone();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s.clone());
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            if current < result {
                result = current.clone();
            }
            let mut next = current.clone();
            let mut a = next.chars().collect::<Vec<char>>();
            for i in (1..next.len()).step_by(2) {
                let digit = a[i].to_digit(10).unwrap();
                a[i] = ((digit + b) % 10).to_string().chars().next().unwrap();
            }
            let next = a.iter().collect::<String>();
            queue.push_back(next.clone());
            let mut a = next.chars().collect::<Vec<char>>();
            for i in 0..next.len() {
                if i % 2 == 1 {
                    let digit = a[i].to_digit(10).unwrap();
                    a[i] = ((digit + b) % 10).to_string().chars().next().unwrap();
                }
            }
            let next = a.iter().collect::<String>();
            queue.push_back(next.clone());
            let mut a = next.chars().collect::<Vec<char>>();
            for i in 0..next.len() {
                let digit = a[i].to_digit(10).unwrap();
                a[i] = ((digit + a) % 10).to_string().chars().next().unwrap();
            }
            let next = a.iter().collect::<String>();
            queue.push_back(next.clone());
        }
        result
    }
}
