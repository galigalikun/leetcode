fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
    assert_eq!(Solution::longest_palindrome("abcba".to_string()), "abcba".to_string());
    assert_eq!(Solution::longest_palindrome("jrjnbctoqgzimtoklkxcknwmhiztomaofwwzjnhrijwkgmwwuazcowskjhitejnvtblqyepxispasrgvgzqlvrmvhxusiqqzzibcyhpnruhrgbzsmlsuacwptmzxuewnjzmwxbdzqyvsjzxiecsnkdibudtvthzlizralpaowsbakzconeuwwpsqynaxqmgngzpovauxsqgypinywwtmekzhhlzaeatbzryreuttgwfqmmpeywtvpssznkwhzuqewuqtfuflttjcxrhwexvtxjihunpywerkktbvlsyomkxuwrqqmbmzjbfytdddnkasmdyukawrzrnhdmaefzltddipcrhuchvdcoegamlfifzistnplqabtazunlelslicrkuuhosoyduhootlwsbtxautewkvnvlbtixkmxhngidxecehslqjpcdrtlqswmyghmwlttjecvbueswsixoxmymcepbmuwtzanmvujmalyghzkvtoxynyusbpzpolaplsgrunpfgdbbtvtkahqmmlbxzcfznvhxsiytlsxmmtqiudyjlnbkzvtbqdsknsrknsykqzucevgmmcoanilsyyklpbxqosoquolvytefhvozwtwcrmbnyijbammlzrgalrymyfpysbqpjwzirsfknnyseiujadovngogvptphuyzkrwgjqwdhtvgxnmxuheofplizpxijfytfabx".to_string()), "qosoq".to_string());
    assert_eq!(Solution::longest_palindrome("azwdzwmwcqzgcobeeiphemqbjtxzwkhiqpbrprocbppbxrnsxnwgikiaqutwpftbiinlnpyqstkiqzbggcsdzzjbrkfmhgtnbujzszxsycmvipjtktpebaafycngqasbbhxaeawwmkjcziybxowkaibqnndcjbsoehtamhspnidjylyisiaewmypfyiqtwlmejkpzlieolfdjnxntonnzfgcqlcfpoxcwqctalwrgwhvqvtrpwemxhirpgizjffqgntsmvzldpjfijdncexbwtxnmbnoykxshkqbounzrewkpqjxocvaufnhunsmsazgibxedtopnccriwcfzeomsrrangufkjfzipkmwfbmkarnyyrgdsooosgqlkzvorrrsaveuoxjeajvbdpgxlcrtqomliphnlehgrzgwujogxteyulphhuhwyoyvcxqatfkboahfqhjgujcaapoyqtsdqfwnijlkknuralezqmcryvkankszmzpgqutojoyzsnyfwsyeqqzrlhzbc".to_string()), "sooos".to_string());
}

pub struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        for n in (2..=s.len()).rev() {
            for i in 0..=s.len()-n {
                let p = s.as_str().chars().skip(i).take(n).collect::<String>();
                if p == p.chars().rev().collect::<String>() {
                    return p;
                }
            }
        }

        return s.as_str().chars().skip(0).take(1).collect::<String>();
    }
}
