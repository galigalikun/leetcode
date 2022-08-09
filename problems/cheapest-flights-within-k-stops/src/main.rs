fn main() {
    assert_eq!(Solution::find_cheapest_price(4, vec![[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], 2, 3, 1), 700);
    assert_eq!(Solution::find_cheapest_price(3, vec![[0,1,100],[1,2,100],[0,2,500]], 0, 2, 1), 200);
    assert_eq!(Solution::find_cheapest_price(3, vec![[0,1,100],[1,2,100],[0,2,500]], 0, 2, 0), 500);
}

struct Solution{}
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        return -1;
    }
}
