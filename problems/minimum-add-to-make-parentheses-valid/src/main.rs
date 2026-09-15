struct Solution;
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open = 0;
        let mut insertions = 0;

        for ch in s.chars() {
            if ch == '(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                insertions += 1;
            }
        }

        insertions + open
    }
}

fn main() {
    let s = "()))((".to_string();
    let answer = Solution::min_add_to_make_valid(s);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_add_to_make_valid("(((((".to_string()), 5);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::min_add_to_make_valid("()".to_string()), 0);
    }

    #[test]
    fn mixed_unbalanced() {
        assert_eq!(Solution::min_add_to_make_valid("()))((".to_string()), 4);
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::min_add_to_make_valid(String::new()), 0);
    }
}
