fn main() {
    assert_eq!(Solution::basic_calculator_iv("e + 8 - a + 5".to_string(), vec!["e".to_string()], vec![1]), vec!["-1*a","14"]);
    assert_eq!(Solution::basic_calculator_iv("e - 8 + temperature - pressure".to_string(), vec!["e".to_string(), "temperature".to_string()], vec![1, 12]), vec!["-1*pressure","5"]);
    assert_eq!(Solution::basic_calculator_iv("(e + 8) * (e - 8)".to_string(), vec![], vec![]), vec!["1*e*e","-64"]);
}

use std::collections::HashMap;

struct Solution {}

#[derive(Clone, Debug, Default)]
struct Poly {
    // key: sorted variables in a monomial; empty means constant term.
    terms: HashMap<Vec<String>, i32>,
}

impl Poly {
    fn from_int(value: i32) -> Self {
        let mut terms = HashMap::new();
        if value != 0 {
            terms.insert(Vec::new(), value);
        }
        Self { terms }
    }

    fn from_var(name: String, eval: &HashMap<String, i32>) -> Self {
        match eval.get(&name) {
            Some(value) => Self::from_int(*value),
            None => {
                let mut terms = HashMap::new();
                terms.insert(vec![name], 1);
                Self { terms }
            }
        }
    }

    fn add(&self, other: &Self, sign: i32) -> Self {
        let mut out = self.terms.clone();
        for (mono, coeff) in &other.terms {
            let entry = out.entry(mono.clone()).or_insert(0);
            *entry += sign * coeff;
        }
        out.retain(|_, coeff| *coeff != 0);
        Self { terms: out }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut out = HashMap::new();
        for (lhs_mono, lhs_coeff) in &self.terms {
            for (rhs_mono, rhs_coeff) in &other.terms {
                let mut mono = lhs_mono.clone();
                mono.extend(rhs_mono.iter().cloned());
                mono.sort();
                let entry = out.entry(mono).or_insert(0);
                *entry += lhs_coeff * rhs_coeff;
            }
        }
        out.retain(|_, coeff| *coeff != 0);
        Self { terms: out }
    }

    fn to_tokens(&self) -> Vec<String> {
        let mut items: Vec<(Vec<String>, i32)> = self
            .terms
            .iter()
            .map(|(mono, coeff)| (mono.clone(), *coeff))
            .collect();

        items.sort_by(|a, b| {
            if a.0.len() != b.0.len() {
                return b.0.len().cmp(&a.0.len());
            }
            a.0.cmp(&b.0)
        });

        items
            .into_iter()
            .map(|(mono, coeff)| {
                if mono.is_empty() {
                    coeff.to_string()
                } else {
                    format!("{}*{}", coeff, mono.join("*"))
                }
            })
            .collect()
    }
}

struct Parser<'a> {
    tokens: Vec<String>,
    idx: usize,
    eval: &'a HashMap<String, i32>,
}

impl<'a> Parser<'a> {
    fn new(expression: &str, eval: &'a HashMap<String, i32>) -> Self {
        Self {
            tokens: tokenize(expression),
            idx: 0,
            eval,
        }
    }

    fn parse_expression(&mut self) -> Poly {
        let mut left = self.parse_term();
        while self.idx < self.tokens.len() {
            let op = self.tokens[self.idx].clone();
            if op != "+" && op != "-" {
                break;
            }
            self.idx += 1;
            let right = self.parse_term();
            left = if op == "+" {
                left.add(&right, 1)
            } else {
                left.add(&right, -1)
            };
        }
        left
    }

    fn parse_term(&mut self) -> Poly {
        let mut left = self.parse_factor();
        while self.idx < self.tokens.len() {
            if self.tokens[self.idx] != "*" {
                break;
            }
            self.idx += 1;
            let right = self.parse_factor();
            left = left.mul(&right);
        }
        left
    }

    fn parse_factor(&mut self) -> Poly {
        let token = self.tokens[self.idx].clone();
        self.idx += 1;

        if token == "(" {
            let inside = self.parse_expression();
            self.idx += 1; // skip ')'
            return inside;
        }

        if token.chars().next().unwrap().is_ascii_digit() {
            return Poly::from_int(token.parse::<i32>().unwrap());
        }

        Poly::from_var(token, self.eval)
    }
}

fn tokenize(expression: &str) -> Vec<String> {
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;
    let mut tokens = Vec::new();

    while i < chars.len() {
        let c = chars[i];
        if c == ' ' {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let mut j = i;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            tokens.push(chars[i..j].iter().collect());
            i = j;
            continue;
        }
        if c.is_ascii_lowercase() {
            let mut j = i;
            while j < chars.len() && chars[j].is_ascii_lowercase() {
                j += 1;
            }
            tokens.push(chars[i..j].iter().collect());
            i = j;
            continue;
        }

        tokens.push(c.to_string());
        i += 1;
    }

    tokens
}

impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        let eval: HashMap<String, i32> = evalvars.into_iter().zip(evalints.into_iter()).collect();
        let mut parser = Parser::new(&expression, &eval);
        parser.parse_expression().to_tokens()
    }
}
