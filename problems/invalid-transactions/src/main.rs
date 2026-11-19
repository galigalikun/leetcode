fn main() {
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"alice,50,100,beijing".to_string()]), vec!["alice,20,800,mtv","alice,50,100,beijing"]);
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"alice,50,1200,mtv".to_string()]), vec!["alice,50,1200,mtv"]);
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"bob,50,1200,mtv".to_string()]), vec!["bob,50,1200,mtv"]);
    assert_eq!(Solution::invalid_transactions(vec!["alice,0,800,mtv".to_string(),"alice,60,800,beijing".to_string(),"alice,61,800,mtv".to_string()]), vec!["alice,0,800,mtv","alice,60,800,beijing","alice,61,800,mtv"]);
}

struct Solution;
struct Transaction {
    name: String,
    time: i32,
    amount: i32,
    city: String,
}
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let parsed: Vec<Transaction> = transactions
            .iter()
            .map(|transaction| {
                let fields: Vec<&str> = transaction.split(',').collect();
                Transaction {
                    name: fields[0].to_string(),
                    time: fields[1].parse().unwrap(),
                    amount: fields[2].parse().unwrap(),
                    city: fields[3].to_string(),
                }
            })
            .collect();

        let mut invalid = vec![false; parsed.len()];

        for i in 0..parsed.len() {
            if parsed[i].amount > 1000 {
                invalid[i] = true;
            }

            for j in 0..parsed.len() {
                if i == j {
                    continue;
                }
                if parsed[i].name == parsed[j].name
                    && parsed[i].city != parsed[j].city
                    && (parsed[i].time - parsed[j].time).abs() <= 60
                {
                    invalid[i] = true;
                    invalid[j] = true;
                }
            }
        }

        transactions
            .into_iter()
            .enumerate()
            .filter_map(|(i, transaction)| invalid[i].then_some(transaction))
            .collect()
    }
}
