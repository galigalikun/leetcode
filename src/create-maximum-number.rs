fn main() {
    assert_eq!(
        Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );
}

pub struct Solution {}
impl Solution {
    fn max(nums:Vec<i32>, k: i32) -> Vec<i32> {
        let mut drop = nums.len() as i32 - k;
        let mut result = vec![];
        for n in nums {
            while drop < 0 && !result.is_empty() && result.last() < Some(&n) {
                result.pop();
                drop -= 1;
            }
            result.push(n);
        }

        return result;
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![0; k as usize];
        for i in std::cmp::max(0, k - nums2.len() as i32)..=std::cmp::min(k, nums1.len() as i32) {
            println!("debug1 {:?}", Solution::max(nums1.clone(), i));
            println!("debug2 {:?}", Solution::max(nums2.clone(), k-i));
        }
        println!("debug {:?}", result);
        return vec![];
    }
}
