fn main() {
    assert_eq!(Solution::largest_palindrome(2), 987);
    assert_eq!(Solution::largest_palindrome(1), 9);
    assert_eq!(Solution::largest_palindrome(5), 677);
    assert_eq!(Solution::largest_palindrome(7), 877);
}

struct Solution {}
// https://jinglescode.github.io/2020/01/07/project-euler-problem-4/
impl Solution {
    fn is_palindrome(initial_number: i64) -> bool {
        let mut reversed = 0;
        let mut temp = initial_number;
        while temp > 0 {
            let last_digit = temp % 10;
            reversed = reversed * 10 + last_digit;
            temp = temp / 10;
        }
        return initial_number == reversed;
    }
    pub fn largest_palindrome(n: i32) -> i32 {
        let largest_number = 10_i32.pow(n as u32);
        let mut smallest_number = 10_i32.pow((n - 1) as u32);
        let mut largest_palindrome = 0;
        for outer_i in (smallest_number..=largest_number).rev() {
            for inner_i in (smallest_number..=outer_i).rev() {
                let number = outer_i as i64 * inner_i as i64;
                if Solution::is_palindrome(number) && number > largest_palindrome {
                    largest_palindrome = number;
                    if inner_i > smallest_number {
                        smallest_number = inner_i;
                    }
                    break;
                }
            }
        }
        return (largest_palindrome % 1337) as i32;
    }
}
