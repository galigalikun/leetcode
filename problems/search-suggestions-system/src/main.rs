fn main() {
    assert_eq!(
        Solution::suggested_products(
            vec![
                "mobile".to_string(),
                "mouse".to_string(),
                "moneypot".to_string(),
                "monitor".to_string(),
                "mousepad".to_string()
            ],
            "mouse".to_string()
        ),
        vec![
            vec!["mobile", "moneypot", "monitor"],
            vec!["mobile", "moneypot", "monitor"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"]
        ]
    );
    assert_eq!(
        Solution::suggested_products(vec!["havana".to_string()], "havana".to_string()),
        vec![
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"]
        ]
    );
}

struct Solution;
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let mut result = vec![];
        let search_word = search_word;
        for i in 0..search_word.len() {
            let mut temp = vec![];
            for j in 0..products.len() {
                if products[j].len() >= i + 1 {
                    if products[j][0..i + 1] == search_word[0..i + 1] {
                        temp.push(products[j].clone());
                    }
                }
            }
            if temp.len() > 3 {
                result.push(temp[0..3].to_vec());
            } else {
                result.push(temp);
            }
        }
        return result;
    }
}
