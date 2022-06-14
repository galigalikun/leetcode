fn main() {
    assert_eq!(Solution::count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(Solution::count_of_atoms(String::from("Mg(OH)2")), String::from("H2MgO2"));
}

struct Solution{}
use std::collections::HashMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut map = HashMap::new();
        let mut stack = Vec::new();
        let mut i = 0;
        while i < formula.len() {
            if formula.chars().nth(i) == Some('(') {
                stack.push(i);
            } else if formula.chars().nth(i) == Some(')') {
                let start = stack.pop().unwrap();
                let mut atom = String::new();
                for j in start + 1..i {
                    atom.push(formula.chars().nth(j).unwrap());
                }
                let mut count = 1;
                while i + 1 < formula.len() && formula.chars().nth(i + 1) >= Some('0') && formula.chars().nth(i + 1) <= Some('9') {
                    count *= formula.chars().nth(i + 1).unwrap() as u32 - '0' as u32;
                    i += 1;
                }
                if let Some(v) = map.get_mut(&atom) {
                    *v += count;
                } else {
                    map.insert(atom, count);
                }
            }
            i += 1;
        }
        println!("{:?}", map);
        let mut res = String::new();
        for (k, v) in map.iter() {
            res.push_str(&format!("{}{}", v, k));
        }
        return res;
    }
}
