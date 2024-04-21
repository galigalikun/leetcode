fn main() {
    assert_eq!(
        Solution::display_table(vec![
            vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
            vec![
                "Corina".to_string(),
                "10".to_string(),
                "Beef Burrito".to_string()
            ],
            vec![
                "David".to_string(),
                "3".to_string(),
                "Fried Chicken".to_string()
            ],
            vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
            vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()]
        ]),
        vec![
            ["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
            ["3", "0", "2", "1", "0"],
            ["5", "0", "1", "0", "1"],
            ["10", "1", "0", "0", "0"]
        ]
    );
    assert_eq!(
        Solution::display_table(vec![
            vec![
                "James".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string()
            ],
            vec![
                "Ratesh".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string()
            ],
            vec![
                "Amadeus".to_string(),
                "12".to_string(),
                "Fried Chicken".to_string()
            ],
            vec![
                "Adam".to_string(),
                "1".to_string(),
                "Canadian Waffles".to_string()
            ],
            vec![
                "Brianna".to_string(),
                "1".to_string(),
                "Canadian Waffles".to_string()
            ]
        ]),
        vec![
            ["Table", "Canadian Waffles", "Fried Chicken"],
            ["1", "2", "0"],
            ["12", "0", "3"]
        ]
    );
    assert_eq!(
        Solution::display_table(vec![
            vec![
                "Laura".to_string(),
                "2".to_string(),
                "Bean Burrito".to_string()
            ],
            vec![
                "Jhon".to_string(),
                "2".to_string(),
                "Beef Burrito".to_string()
            ],
            vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()]
        ]),
        vec![
            ["Table", "Bean Burrito", "Beef Burrito", "Soda"],
            ["2", "1", "1", "1"]
        ]
    );
}

struct Solution;
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut table = HashMap::new();
        let mut foods = HashMap::new();
        for order in &orders {
            let table_no = order[1].parse::<i32>().unwrap();
            let food = order[2].clone();
            *table
                .entry(table_no)
                .or_insert(HashMap::new())
                .entry(food.clone())
                .or_insert(0) += 1;
            foods.entry(food).or_insert(0);
        }
        let mut foods = foods.keys().map(|x| x.clone()).collect::<Vec<String>>();
        foods.sort();
        let mut result = vec![vec!["Table".to_string()]];
        for food in &foods {
            result[0].push(food.clone());
        }
        foods.sort();
        let mut tables = table.keys().map(|x| *x).collect::<Vec<i32>>();
        tables.sort();
        for table_no in tables {
            let mut row = vec![table_no.to_string()];
            for food in &foods {
                row.push(
                    table
                        .get(&table_no)
                        .unwrap()
                        .get(food)
                        .unwrap_or(&0)
                        .to_string(),
                );
            }
            result.push(row);
        }
        return result;
    }
}
