fn main() {
    assert_eq!(
        Solution::array_rank_transform(vec![40, 10, 20, 30]),
        vec![4, 1, 2, 3]
    );
    assert_eq!(
        Solution::array_rank_transform(vec![100, 100, 100]),
        vec![1, 1, 1]
    );
    assert_eq!(
        Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}

struct Solution;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();
        println!("{:?}", arr);
        let mut map = std::collections::HashMap::new();
        let mut rank = 1;
        for i in 0..arr.len() {
            if !map.contains_key(&arr[i]) {
                map.insert(arr[i], rank);
                rank += 1;
            }
        }
        let mut result = vec![];
        for i in 0..arr.len() {
            result.push(*map.get(&arr[i]).unwrap());
        }
        return result;
    }
}
