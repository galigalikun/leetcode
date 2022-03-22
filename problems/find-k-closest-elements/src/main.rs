fn main() {
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9),
        vec![10]
    );
}

// https://www.techiedelight.com/find-k-closest-elements-to-given-value-array/
struct Solution {}
impl Solution {
    fn helper(arr: Vec<i32>, n: usize, x: i32) -> usize {
        let mut low = 0;
        let mut high = n as i32 - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if arr[mid as usize] < x {
                low = mid + 1;
            } else if arr[mid as usize] > x {
                high = mid - 1;
            } else {
                return mid as usize;
            }
        }
        return low as usize;
    }
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // a - x < b - x
        // 1 - 3 < 2 - 3
        // -2 < -1
        let i = Solution::helper(arr.clone(), arr.len(), x);
        let mut left = i as i32 - 1;
        let mut right = i;
        let mut kk = k;
        while kk > 0 {
            if left < 0 {
                right += 1;
            } else if right < arr.len() && (arr[left as usize] - x).abs() > (arr[right] - x).abs() {
                right += 1;
            } else {
                left -= 1;
            }
            kk -= 1;
        }
        let mut ans = vec![];
        let mut ll = if left > 0 { left as usize } else { 0 };
        while ll < right {
            ans.push(arr[ll]);
            ll += 1;
        }

        return ans;
    }
}
