fn main() {
    assert_eq!(
        Solution::longest_palindrome("babad".to_string()),
        "bab".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("cbbd".to_string()),
        "bb".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("a".to_string()),
        "a".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("ac".to_string()),
        "a".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("abcba".to_string()),
        "abcba".to_string()
    );
    assert_eq!(Solution::longest_palindrome("jrjnbctoqgzimtoklkxcknwmhiztomaofwwzjnhrijwkgmwwuazcowskjhitejnvtblqyepxispasrgvgzqlvrmvhxusiqqzzibcyhpnruhrgbzsmlsuacwptmzxuewnjzmwxbdzqyvsjzxiecsnkdibudtvthzlizralpaowsbakzconeuwwpsqynaxqmgngzpovauxsqgypinywwtmekzhhlzaeatbzryreuttgwfqmmpeywtvpssznkwhzuqewuqtfuflttjcxrhwexvtxjihunpywerkktbvlsyomkxuwrqqmbmzjbfytdddnkasmdyukawrzrnhdmaefzltddipcrhuchvdcoegamlfifzistnplqabtazunlelslicrkuuhosoyduhootlwsbtxautewkvnvlbtixkmxhngidxecehslqjpcdrtlqswmyghmwlttjecvbueswsixoxmymcepbmuwtzanmvujmalyghzkvtoxynyusbpzpolaplsgrunpfgdbbtvtkahqmmlbxzcfznvhxsiytlsxmmtqiudyjlnbkzvtbqdsknsrknsykqzucevgmmcoanilsyyklpbxqosoquolvytefhvozwtwcrmbnyijbammlzrgalrymyfpysbqpjwzirsfknnyseiujadovngogvptphuyzkrwgjqwdhtvgxnmxuheofplizpxijfytfabx".to_string()), "qosoq".to_string());
    assert_eq!(Solution::longest_palindrome("azwdzwmwcqzgcobeeiphemqbjtxzwkhiqpbrprocbppbxrnsxnwgikiaqutwpftbiinlnpyqstkiqzbggcsdzzjbrkfmhgtnbujzszxsycmvipjtktpebaafycngqasbbhxaeawwmkjcziybxowkaibqnndcjbsoehtamhspnidjylyisiaewmypfyiqtwlmejkpzlieolfdjnxntonnzfgcqlcfpoxcwqctalwrgwhvqvtrpwemxhirpgizjffqgntsmvzldpjfijdncexbwtxnmbnoykxshkqbounzrewkpqjxocvaufnhunsmsazgibxedtopnccriwcfzeomsrrangufkjfzipkmwfbmkarnyyrgdsooosgqlkzvorrrsaveuoxjeajvbdpgxlcrtqomliphnlehgrzgwujogxteyulphhuhwyoyvcxqatfkboahfqhjgujcaapoyqtsdqfwnijlkknuralezqmcryvkankszmzpgqutojoyzsnyfwsyeqqzrlhzbc".to_string()), "sooos".to_string());
}

pub struct Solution {}

impl Solution {
    // https://en.wikipedia.org/wiki/Longest_palindromic_substring
    pub fn add_boundaries(cs: Vec<char>) -> Vec<char> {
        if cs.len() == 0 {
            return "||".chars().collect();
        }

        let mut cs2 = Vec::new();

        for i in (0..cs.len() * 2).step_by(2) {
            cs2.push('|');
            cs2.push(cs[i / 2]);
        }
        cs2.push('|');
        return cs2;
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let s2 = Solution::add_boundaries(s.as_str().chars().collect());
        let mut p: Vec<i32> = Vec::with_capacity(s2.len());
        for i in 0..s2.len() {
            p.push(0);
        }
        let mut c = 0;
        let mut r: i32 = 0; // Here the first element in s2 has been processed.
        let mut m: i32 = 0;
        let mut n = 0; // The walking indices to compare if two elements are the same.

        for i in 1..s2.len() as i32 {
            if i > r {
                p[i as usize] = 0;
                m = i - 1;
                n = i + 1;
            } else {
                let i2 = c * 2 - i;
                if p[i2 as usize] < (r - i - 1) as i32 {
                    p[i as usize] = p[i2 as usize];
                    m = -1; // This signals bypassing the while loop below.
                } else {
                    p[i as usize] = (r - i) as i32;
                    n = r + 1;
                    m = i as i32 * 2 - n as i32;
                }
            }

            while m >= 0 && n < s2.len() as i32 && s2[m as usize] == s2[n as usize] {
                p[i as usize] += 1;
                m -= 1;
                n += 1;
            }
            if i + p[i as usize] > r {
                c = i;
                r = i + p[i as usize];
            }
        }
        let mut len: usize = 0;
        let mut c: usize = 0;
        for i in 1..s2.len() {
            if (len as i32) < p[i] {
                len = p[i] as usize;
                c = i;
            }
        }
        return s2
            .iter()
            .skip(c - len)
            .take(len + len + 1)
            .filter(|&x| x != &'|')
            .collect::<String>();
    }
}
