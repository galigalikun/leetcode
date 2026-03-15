fn main() {
    assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 5);
}

struct Solution;
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut count = [0; 5];
        for num in nums {
            count[num as usize - 1] += 1;
        }
        for i in (0..4).rev() {
            if count[i] < count[i + 1] {
                count[i] = count[i + 1];
            }
        }
        for i in 0..4 {
            if count[i] > count[i + 1] {
                count[i + 1] = count[i];
            }
        }
        for i in (0..4).rev() {
            if count[i] < count[i + 1] {
                count[i] = count[i + 1];
            }
        }
        for i in 0..4 {
            if count[i] > count[i + 1] {
                count[i + 1] = count[i];
            }
        }
        return count.iter().sum::<i32>();
    }
}
