fn main() {
    assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
    assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
    assert_eq!(
        Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut max_profit = 0;
        let mut max_profit_round = 0;
        let mut profit = 0;
        let mut round = 0;
        let mut waiting = 0;
        let mut i = 0;
        while i < customers.len() || waiting > 0 {
            if i < customers.len() {
                waiting += customers[i];
            }
            let board = waiting.min(4);
            waiting -= board;
            profit += board * boarding_cost - running_cost;
            round += 1;
            if profit > max_profit {
                max_profit = profit;
                max_profit_round = round;
            }
            i += 1;
        }
        if max_profit <= 0 {
            return -1;
        }
        let mut max_profit_round_board = 0;
        for i in 0..max_profit_round {
            max_profit_round_board += customers[i];
        }
        let max_profit_round_board_profit_round_profit = 0;
        let mut max_profit_round_board_profit_round_profit_round = 0;
        for i in 0..max_profit_round {
            let max_profit_round_board_cost = max_profit_round_board.min(4) * boarding_cost;
            let max_profit_round_board_profit = max_profit_round_board_cost - running_cost;
            if max_profit_round_board_profit_round_profit_round < max_profit_round_board_profit {
                max_profit_round_board_profit_round_profit_round = max_profit_round_board_profit;
            }
        }
        max_profit_round_board_profit_round_profit_round +=
            max_profit_round_board_profit_round_profit;
        if max_profit_round_board_profit_round_profit_round > max_profit {
            return max_profit_round_board_profit_round_profit_round;
        }
        max_profit
    }
}
