fn main() {
    assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    assert_eq!(
        Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
        vec![0, 0, 1, 1, 2, 5]
    );
}

struct Solution;
impl Solution {
    fn sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }
        let pivot = nums[left];
        let mut i = left;
        let mut j = right;
        loop {
            while nums[i] < pivot {
                i += 1;
            }
            while nums[j] > pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            nums[i] = nums[i] ^ nums[j];
            nums[j] = nums[i] ^ nums[j];
            nums[i] = nums[i] ^ nums[j];
            i += 1;
            j -= 1;
        }
        if i > 0 {
            Self::sort(nums, left, i - 1);
        }
        Self::sort(nums, j + 1, right);
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums;
        let r = ans.len() - 1;
        Self::sort(&mut ans, 0, r);
        return ans;
    }
}
