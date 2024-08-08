fn main() {
    assert_eq!(Solution::max_sum(vec![2,4,5,8,10], vec![4,6,8,9]), 30);
    assert_eq!(Solution::max_sum(vec![1,3,5,7,9], vec![3,5,100]), 109);
    assert_eq!(Solution::max_sum(vec![1,2,3,4,5], vec![6,7,8,9,10]), 40);
}

struct Solution;
impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut result = 0;
        let modu = 1000000007;
        for i in 0..nums1.len() {
            sum1 += nums1[i];
        }
        for i in 0..nums2.len() {
            sum2 += nums2[i];
        }
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                result += nums1[i];
                i += 1;
            } else if nums1[i] > nums2[j] {
                result += nums2[j];
                j += 1;
            } else {
                result += std::cmp::max(sum1, sum2);
                result += nums1[i];
                result = result % modu;
                sum1 = 0;
                sum2 = 0;
                i += 1;
                j += 1;
            }
        }
        while i < nums1.len() {
            result += nums1[i];
            i += 1;
        }
        while j < nums2.len() {
            result += nums2[j];
            j += 1;
        }
        result = result % modu;
        return result;
    }
}
