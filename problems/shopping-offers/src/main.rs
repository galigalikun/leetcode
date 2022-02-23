fn main() {
    assert_eq!(
        Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]),
        14
    );
    assert_eq!(
        Solution::shopping_offers(
            vec![2, 3, 4],
            vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
            vec![1, 2, 1]
        ),
        11
    );
}

//https://github.com/YaokaiYang-assaultmaster/LeetCode/blob/master/LeetcodeAlgorithmQuestions/638.%20Shopping%20Offers.md
struct Solution {}
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut p = 0;
        for i in 0..needs.len() {
            p += price[i] * needs[i];
        }
        let mut res = vec![p];
        for i in 0..special.len() {
            let mut b = true;
            for j in 0..needs.len() {
                b &= special[i][j] <= needs[j];
            }
            if b {
                let mut clone = needs.clone();
                for j in 0..clone.len() {
                    clone[j] = clone[j] - special[i][j];
                }
                res.push(
                    special[i].last().unwrap()
                        + Solution::shopping_offers(price.clone(), special.clone(), clone),
                );
            }
        }
        return *res.iter().min().unwrap();
    }
}
