fn main() {
    assert_eq!(
        Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
        vec![9, 8, 6, 5, 3]
    );

    assert_eq!(
        Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
        vec![6, 7, 6, 0, 4]
    );

    assert_eq!(
        Solution::max_number(vec![3, 9], vec![8, 9], 3),
        vec![9, 8, 9]
    );
}

struct Solution {}
// https://www.programmersought.com/article/27022206431/
impl Solution {
    fn max(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut drop = nums.len() as i32 - k;
        let mut result = vec![];
        for n in nums {
            while drop > 0 && !result.is_empty() && result.last() < Some(&n) {
                result.pop();
                drop -= 1;
            }
            result.push(n);
        }

        result.resize(k as usize, 0);

        return result;
    }

    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut n1 = nums1;
        let mut n2 = nums2;
        while !n1.is_empty() || !n2.is_empty() {
            if n1 > n2 {
                result.push(n1.remove(0));
            } else {
                result.push(n2.remove(0));
            }
        }
        return result;
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![0; k as usize];
        for i in std::cmp::max(0, k - nums2.len() as i32)..=std::cmp::min(k, nums1.len() as i32) {
            let n1 = Solution::max(nums1.clone(), i);
            let n2 = Solution::max(nums2.clone(), k - i);
            let m = Solution::merge(n1, n2);
            result = std::cmp::max(result, m);
        }
        return result;
    }
}
