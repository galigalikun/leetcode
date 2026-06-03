fn main() {
    assert_eq!(Solution::maximum_swap(2736), 7236);
    assert_eq!(Solution::maximum_swap(9973), 9973);
}

struct Solution {}
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num = num;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();

        let mut last = [0; 10];
        for (i, &d) in digits.iter().enumerate() {
            last[d as usize] = i;
        }

        for (i, &d) in digits.iter().enumerate() {
            for j in (d + 1..=9).rev() {
                if last[j as usize] > i {
                    digits.swap(i, last[j as usize]);
                    return digits.into_iter().fold(0, |acc, d| acc * 10 + d);
                }
            }
        }
        digits.into_iter().fold(0, |acc, d| acc * 10 + d)
    }
}
