fn main() {
    let mut nums = vec![1, 2, 3];

    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 3, 2]);

    nums = vec![3, 2, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3]);

    nums = vec![1, 1, 5];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 5, 1]);

    nums = vec![1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1]);

    nums = vec![4, 1, 3, 7, 6, 5, 2];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![4, 1, 5, 2, 3, 6, 7]);
}

struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k: Option<usize> = None;
        for i in 0..nums.len() - 1 {
            if nums[i + 1] > nums[i] {
                k = Some(i);
            }
        }

        if let Some(a) = k {
            let mut l: Option<usize> = None;
            for i in a..nums.len() {
                if nums[i] > nums[a] {
                    l = Some(i);
                }
            }
            if let Some(b) = l {
                let n1 = nums[a];
                let n2 = nums[b];
                nums[a] = n2;
                nums[b] = n1;
            }
            let max = nums.len();
            let mut j = 1;
            for i in a + 1..a + 1 + (nums.len() - a - 1) / 2 {
                let n1 = nums[i];
                let n2 = nums[nums.len() - j];
                nums[i] = n2;
                nums[max - j] = n1;
                j += 1;
            }
        } else {
            let max = nums.len();
            for i in 0..nums.len() / 2 {
                let n1 = nums[i];
                let n2 = nums[nums.len() - i - 1];
                nums[i] = n2;
                nums[max - i - 1] = n1;
            }
        }
    }
}
