fn main() {
    let mut nums = vec![1, 5, 1, 1, 6, 4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, vec![1, 5, 1, 6, 1, 4]);

    let mut nums = vec![1, 3, 2, 2, 3, 1];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, vec![2, 3, 1, 3, 1, 2]);

    let mut nums = vec![1, 5, 2, 5, 4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, vec![4, 5, 1, 5, 2]);
}

pub struct Solution {}
// https://evelynn.gitbooks.io/google-interview/content/wiggle_sort_ii.html
impl Solution {
    fn partition(nums: &mut Vec<i32>, left: i32, right: i32) -> i32 {
        let mut i = left;
        let mut j = right + 1;
        loop {
            while {
                i += 1;
                i < right && nums[i as usize] < nums[left as usize]
            } {}

            while {
                j -= 1;
                j > left && nums[left as usize] < nums[j as usize]
            } {}

            if i >= j {
                break;
            }
            nums.swap(i as usize, j as usize);
        }
        nums.swap(left as usize, j as usize);
        return j;
    }
    fn find_kth_largest(nums: &mut Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32 - k;

        let mut lo: i32 = 0;
        let mut hi = nums.len() as i32 - 1;
        while lo < hi {
            let j = Solution::partition(nums, lo, hi);
            if j < n {
                lo = j + 1;
            } else if j > n {
                hi = j - 1;
            } else {
                break;
            }
        }
        return nums[n as usize];
    }
    fn new_index(index: i32, n: i32) -> usize {
        return (1 + 2 * index as usize) % (n as usize | 1);
    }
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        // [1,5,1,1,6,4]
        // 1 < 5 > 1 < 1 > 6 < 4
        // 1 < 5 > 1 < 6 > 1 < 4
        // [1,6,1,5,1,4]
        // 1 < 6 > 1 < 5 > 1 < 4
        let median = Solution::find_kth_largest(nums, (nums.len() as i32 + 1) / 2);
        let n = nums.len() as i32;

        let mut left = 0;
        let mut i = 0;
        let mut right = n - 1;
        while i <= right {
            if nums[Solution::new_index(i, n)] > median {
                nums.swap(Solution::new_index(left, n), Solution::new_index(i, n));
                left += 1;
                i += 1;
            } else if nums[Solution::new_index(i, n)] < median {
                nums.swap(Solution::new_index(right, n), Solution::new_index(i, n));
                right -= 1;
            } else {
                i += 1;
            }
        }
    }
}
