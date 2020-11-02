fn main() {
    let v1 = &mut vec![1,1,2];
    assert_eq!(Solution::remove_duplicates(v1), 2);
    assert_eq!(v1, &mut vec![1, 2]);
    let v2 = &mut vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(Solution::remove_duplicates(v2), 5);
    assert_eq!(v2, &mut vec![0,1,2,3,4]);
}

pub struct Solution{}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();

        let mut prev:Option<i32> = None;
        let mut w = 0;
        for i in 0..nums.len() {
            if let Some(p) = prev {
                if p == nums[i-w] {
                    nums.remove(i-w);
                    w += 1;
                }
            }
            prev = Some(nums[i-w]);
        }

        return nums.len() as i32;
    }
}
