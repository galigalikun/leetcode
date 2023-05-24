fn main() {
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ]),
        110
    );
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469]
        ]),
        1859
    );
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42]
        ]),
        3086
    );
}

struct Solution;
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_by(|a, b| a[0].cmp(&b[1]));
        let mut sum = 0;
        for i in 0..costs.len() {
            if i < costs.len() / 2 {
                sum += costs[i][0];
            } else {
                sum += costs[i][1];
            }
        }
        return sum;
    }
}
