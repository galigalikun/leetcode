fn main() {
    assert_eq!(
        Solution::largest_multiple_of_three(vec![8, 1, 9]),
        "981".to_string()
    );
    assert_eq!(
        Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
        "8760".to_string()
    );
    assert_eq!(Solution::largest_multiple_of_three(vec![1]), "".to_string());
}

struct Solution;
impl Solution {
    fn remove(count: &mut [i32; 10], d: i32) -> bool {
        for i in (0..10).rev() {
            if i % 3 == d % 3 && count[i as usize] > 0 {
                count[i as usize] -= 1;
                return true;
            }
        }
        return false;
    }
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut digits = digits;
        digits.sort();
        let mut sum = 0;
        let mut count = [0; 10];
        for &d in &digits {
            sum += d;
            count[d as usize] += 1;
        }
        if sum % 3 == 1 {
            if !Solution::remove(&mut count, 1) {
                Solution::remove(&mut count, 2);
                Solution::remove(&mut count, 2);
            }
        } else if sum % 3 == 2 {
            if !Solution::remove(&mut count, 2) {
                Solution::remove(&mut count, 1);
                Solution::remove(&mut count, 1);
            }
        }
        let mut result = String::new();
        for i in (0..10).rev() {
            result.push_str(&i.to_string().repeat(count[i] as usize));
        }
        if result.starts_with("0") {
            return "0".to_string();
        }
        return result;
    }
}
