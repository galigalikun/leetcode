use std::collections::BTreeMap;

fn main() {
    assert_eq!(Solution::count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(
        Solution::count_of_atoms(String::from("Mg(OH)2")),
        String::from("H2MgO2")
    );
    assert_eq!(
        Solution::count_of_atoms(String::from("K4(ON(SO3)2)2")),
        String::from("K4N2O14S4")
    );
}

struct Solution {}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let chars: Vec<char> = formula.chars().collect();
        let mut index = 0;
        let counts = Self::parse_formula(&chars, &mut index);

        let mut result = String::new();
        for (atom, count) in counts {
            result.push_str(&atom);
            if count > 1 {
                result.push_str(&count.to_string());
            }
        }

        result
    }

    fn parse_formula(chars: &[char], index: &mut usize) -> BTreeMap<String, i32> {
        let mut counts = BTreeMap::new();

        while *index < chars.len() && chars[*index] != ')' {
            if chars[*index] == '(' {
                *index += 1; // consume '('
                let nested_counts = Self::parse_formula(chars, index);
                *index += 1; // consume ')'

                let multiplier = Self::parse_number(chars, index);
                for (atom, count) in nested_counts {
                    *counts.entry(atom).or_insert(0) += count * multiplier;
                }
            } else {
                let atom = Self::parse_atom(chars, index);
                let count = Self::parse_number(chars, index);
                *counts.entry(atom).or_insert(0) += count;
            }
        }

        counts
    }

    fn parse_atom(chars: &[char], index: &mut usize) -> String {
        let mut atom = String::new();
        atom.push(chars[*index]); // uppercase character
        *index += 1;

        while *index < chars.len() && chars[*index].is_ascii_lowercase() {
            atom.push(chars[*index]);
            *index += 1;
        }

        atom
    }

    fn parse_number(chars: &[char], index: &mut usize) -> i32 {
        let mut number = 0;

        while *index < chars.len() && chars[*index].is_ascii_digit() {
            number = number * 10 + (chars[*index] as i32 - '0' as i32);
            *index += 1;
        }

        if number == 0 { 1 } else { number }
    }
}
