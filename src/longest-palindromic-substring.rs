fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
    assert_eq!(Solution::longest_palindrome("abcba".to_string()), "abcba".to_string());
    assert_eq!(Solution::longest_palindrome("jrjnbctoqgzimtoklkxcknwmhiztomaofwwzjnhrijwkgmwwuazcowskjhitejnvtblqyepxispasrgvgzqlvrmvhxusiqqzzibcyhpnruhrgbzsmlsuacwptmzxuewnjzmwxbdzqyvsjzxiecsnkdibudtvthzlizralpaowsbakzconeuwwpsqynaxqmgngzpovauxsqgypinywwtmekzhhlzaeatbzryreuttgwfqmmpeywtvpssznkwhzuqewuqtfuflttjcxrhwexvtxjihunpywerkktbvlsyomkxuwrqqmbmzjbfytdddnkasmdyukawrzrnhdmaefzltddipcrhuchvdcoegamlfifzistnplqabtazunlelslicrkuuhosoyduhootlwsbtxautewkvnvlbtixkmxhngidxecehslqjpcdrtlqswmyghmwlttjecvbueswsixoxmymcepbmuwtzanmvujmalyghzkvtoxynyusbpzpolaplsgrunpfgdbbtvtkahqmmlbxzcfznvhxsiytlsxmmtqiudyjlnbkzvtbqdsknsrknsykqzucevgmmcoanilsyyklpbxqosoquolvytefhvozwtwcrmbnyijbammlzrgalrymyfpysbqpjwzirsfknnyseiujadovngogvptphuyzkrwgjqwdhtvgxnmxuheofplizpxijfytfabx".to_string()), "qosoq".to_string());
}

pub struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut stack = Vec::new();
        let mut result = "".to_string();
        for i in s.as_str().chars() {
            let v = Vec::new();
            stack.push(v);
            for k in 0..stack.len() {
                stack[k].push(i);
                let p = stack[k].iter().collect::<String>();
                stack[k].reverse();
                let r = stack[k].iter().collect::<String>();
                stack[k].reverse();
                if p == r {
                    if p.len() > result.len() {
                        result = p;
                    }
                }
            }
        }

        return result;
    }
}
