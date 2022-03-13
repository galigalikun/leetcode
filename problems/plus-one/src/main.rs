fn main() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    assert_eq!(Solution::plus_one(vec![2, 4, 9, 3, 9]), vec![2, 4, 9, 4, 0]);
}

struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut n = 0;
        let max = digits.len();
        for i in (0..max).rev() {
            if i == max - 1 {
                let num = digits[i] + 1;
                if num > 9 {
                    n = num / 10;
                    result.push(num % 10);
                    result.push(num / 10);
                } else {
                    result.push(num);
                }
            } else {
                let num = digits[i] + n;
                if n > 0 {
                    if num > 9 {
                        if result.len() == (max - i - 1) {
                            result.push(num % 10);
                            result.push(num / 10);
                        } else {
                            result[max - i - 1] = num % 10;
                            result.push(num / 10);
                        }
                    } else {
                        if result.len() == (max - i - 1) {
                            result.push(num);
                        } else {
                            result[max - i - 1] = num;
                        }
                    }
                } else {
                    result.push(num);
                }
                n = num / 10;
            }
        }
        return result.into_iter().rev().collect::<Vec<i32>>();
    }
}
