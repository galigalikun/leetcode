fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10,15,20]), 15);
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]), 6);
}

struct Solution{}
impl Solution {
    fn helper(i:usize, cost:Vec<i32>) -> i32 {
        return std::cmp::min(cost[i] + if cost.len() > i + 1 {
            cost[i+1]
        } else {
            0
        }, cost[i] + if cost.len() > i + 2 {
            cost[i+2]
    } else {
        0
    });
    }
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let ans1 = std::cmp::min(cost[0] + cost[1], cost[0]+ cost[2]);
        let ans2 = std::cmp::min(cost[1] + cost[2], cost[1]+ cost[3]);
        return std::cmp::min(ans1, ans2);
    }
}
