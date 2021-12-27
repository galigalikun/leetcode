fn main() {
    assert_eq!(Solution::next_greater_element(12), 21);
    assert_eq!(Solution::next_greater_element(21), -1);
    assert_eq!(Solution::next_greater_element(2147483486), -1)
}

// https://aaronice.gitbook.io/lintcode/stack/next-greater-element-iii
struct Solution {}
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut nums = format!("{}", n)
            .chars()
            .map(|x| x as i32 - 48)
            .collect::<Vec<_>>();

        let mut i = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize + 1] <= nums[i as usize] {
            i -= 1;
        }
        if i < 0 {
            return -1;
        }
        let mut j = nums.len() as i32 - 1;
        while j >= 0 && nums[j as usize] <= nums[i as usize] {
            j -= 1;
        }
        nums[i as usize] ^= nums[j as usize];
        nums[j as usize] ^= nums[i as usize];
        nums[i as usize] ^= nums[j as usize];
        let mut start = i + 1;
        let mut end = nums.len() as i32 - 1;
        while start < end {
            nums[start as usize] ^= nums[end as usize];
            nums[end as usize] ^= nums[start as usize];
            nums[start as usize] ^= nums[end as usize];
            start += 1;
            end -= 1;
        }

        return nums
            .iter()
            .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_else(|_e| -1);
    }
}
