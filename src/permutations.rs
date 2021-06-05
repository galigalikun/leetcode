fn main() {
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
}

pub struct Solution {}
// https://programmerstart.com/article/2869306051/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 3 -> 3*2*1
        // 4 -> 4*3*2*1
        // 1 2 3 swap(0, 0)
        // 2 1 3 swap(0, 1)
        // 3 1 2 swap(0, 2)
        // 1 3 2 swap(1, 0)
        // 1 3 2 swap(1, 1)

        // 2 3 1 (2, 1, 3).swap(1, 2)
        // 3 2 1 nums.swap(0, 2)

        // 3 1 2 (3 2 1).swap(1, 2)
        //
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        let cnt = nums.len();
        let mut n = 0;
        while n < cnt {
            let mut tmp = result.clone();
            result.clear();
            for i in 0..tmp.len() {
                for j in 0..cnt {
                    if None == tmp[i].iter().find(|&x| x == &nums[j]) {
                        tmp[i].push(nums[j]);
                        result.push(tmp[i].clone());
                        let idx = tmp[i].len() - 1;
                        tmp[i].remove(idx);
                    }
                }
            }
            n += 1;
        }

        return result;
    }
}
