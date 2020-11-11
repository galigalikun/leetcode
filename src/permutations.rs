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
        let mut result = Vec::new();
        let mut cnt = nums.len();
        while cnt > 0 {
            println!("debugcnt {}", cnt);
            let mut c = nums.clone();
            c.swap(0, nums.len() - cnt);
            for i in 0..cnt {
                let mut c2 = c.clone();
                println!("debug {:?} {} {}", c2, i, cnt - i - 1);
                c2.swap(i, cnt - i - 1);
                result.push(c2);
            }
            cnt -= 1;
        }

        // 1 2 3 4
        // 1 2 3 4
        // 1 3 2 4
        // 1 3 4 2
        // 1 4 2 3
        // 1 4 3 2

        return result;
    }
}
