fn main() {
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"alice,50,100,beijing".to_string()]), vec!["alice,20,800,mtv","alice,50,100,beijing"]);
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"alice,50,1200,mtv".to_string()]), vec!["alice,50,1200,mtv"]);
    assert_eq!(Solution::invalid_transactions(vec!["alice,20,800,mtv".to_string(),"bob,50,1200,mtv".to_string()]), vec!["bob,50,1200,mtv"]);
}

struct Solution;
struct Transaction {
    time: i32,
    amount: i32,
    city: String,
}
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut invalid_transactions = vec![];
        let mut transactions_map = std::collections::HashMap::new();
        for transaction in transactions {
            let transaction: Vec<&str> = transaction.split(",").collect();
            let name = transaction[0];
            let time = transaction[1].parse::<i32>().unwrap();
            let amount = transaction[2].parse::<i32>().unwrap();
            let city = transaction[3];
            if amount > 1000 {
                invalid_transactions.push(transaction.join(","));
            }
            if let a = transactions_map.get(name) {
                for t in a.iter() {
                    if t.city != city && (t.time - time).abs() <= 60 {
                        invalid_transactions.push(transaction.join(","));
                        break;
                    }
                }
                a.push(Transaction {
                    time,
                    amount,
                    city: city.to_string(),
                });
            } else {
                transactions_map.insert(
                    name,
                    vec![Transaction {
                        time,
                        amount,
                        city: city.to_string(),
                    }],
                );
            }
        }
        return invalid_transactions;
    }
}
