fn main() {
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
        vec![4, 5, 2, 7]
    );
    assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
}

struct Solution;
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut tmp_even = vec![];
        let mut tmp_odd = vec![];
        let mut odd = vec![];
        let mut even = vec![];
        for (i, v) in nums.iter().enumerate() {
            if v % 2 == 0 {
                if i % 2 == 0 {
                    ans[i] = *v;
                } else {
                    if let Some(p) = odd.pop() {
                        ans[i] = p;
                    } else {
                        tmp_even.push(i);
                    }
                    if let Some(idx) = tmp_odd.pop() {
                        ans[idx] = *v;
                    } else {
                        even.push(*v);
                    }
                }
            } else {
                if i % 2 == 1 {
                    ans[i] = *v;
                } else {
                    if let Some(p) = even.pop() {
                        ans[i] = p;
                    } else {
                        tmp_odd.push(i);
                    }
                    if let Some(idx) = tmp_even.pop() {
                        ans[idx] = *v;
                    } else {
                        odd.push(*v);
                    }
                }
            }
        }
        return ans;
    }
}
