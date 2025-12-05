fn main() {
    assert_eq!(
        Solution::find_products_of_elements(vec![vec![1, 3, 7]]),
        vec![4]
    );
    assert_eq!(
        Solution::find_products_of_elements(vec![vec![2, 5, 3], vec![7, 7, 4]]),
        vec![2, 2]
    );
}

struct Solution;
impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        return queries
            .iter()
            .map(|query| {
                let product: i64 = query.iter().product();
                (product % 1_000_000_007) as i32
            })
            .collect();
    }
}
