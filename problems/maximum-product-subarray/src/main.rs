fn main() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-2]), -2);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
    assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
}

pub struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max: Option<i32> = None;
        let mut result = nums.clone();

        let mut stock: Vec<i32> = Vec::new();

        let mut product = nums[0];
        for i in 0..nums.len() - 1 {
            let mut work = Vec::new();
            while let Some(w) = stock.pop() {
                work.push(w * nums[i + 1]);
            }
            work.push(nums[i] * nums[i + 1]);

            stock = work.clone();
            result.append(&mut work);
            product *= nums[i];

            if max == None {
                max = Some(product);
            } else if Some(product) > max {
                max = Some(product);
            }

            if Some(nums[i]) > max {
                max = Some(nums[i]);
            }
        }
        if let Some(m) = result.into_iter().max() {
            return m;
        }
        return nums[0];
    }
}
