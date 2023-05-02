fn main() {
    assert_eq!(Solution::smallest_equivalent_string("parker".to_string(), "morris".to_string(), "parser".to_string()), "makkek");
    assert_eq!(Solution::smallest_equivalent_string("hello".to_string(), "world".to_string(), "hold".to_string()), "hdld");
    assert_eq!(Solution::smallest_equivalent_string("leetcode".to_string(), "programs".to_string(), "sourcecode".to_string()), "aauaaaaada");
}

struct Solution;
impl Solution {
    fn find_parent(index: usize, map: &Vec<usize>) -> usize {
        let mut parent = index;
        while parent != map[parent] {
            parent = map[parent];
        }
        return parent;
    }
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let base = base_str.chars().collect::<Vec<char>>();
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut s1_map = vec![0; 26];
        let mut s2_map = vec![0; 26];
        let mut base_map = vec![0; 26];
        let mut result = vec![0; base.len()];
        let mut i = 0;
        while i < s1.len() {
            let s1_char = s1[i];
            let s2_char = s2[i];
            let base_char = base[i];
            let s1_index = (s1_char as u8 - 'a' as u8) as usize;
            let s2_index = (s2_char as u8 - 'a' as u8) as usize;
            let base_index = (base_char as u8 - 'a' as u8) as usize;
            s1_map[s1_index] = s1_index;
            s2_map[s2_index] = s2_index;
            base_map[base_index] = base_index;
            i += 1;
        }
        i = 0;
        while i < s1.len() {
            let s1_char = s1[i];
            let s2_char = s2[i];
            let base_char = base[i];
            let s1_index = (s1_char as u8 - 'a' as u8) as usize;
            let s2_index = (s2_char as u8 - 'a' as u8) as usize;
            let base_index = (base_char as u8 - 'a' as u8) as usize;
            let s1_parent = Solution::find_parent(s1_index, &s1_map);
            let s2_parent = Solution::find_parent(s2_index, &s2_map);
            let base_parent = Solution::find_parent(base_index, &base_map);
            if s1_parent != s2_parent {
                if s1_parent < s2_parent {
                    s2_map[s2_parent] = s1_parent;
                } else {
                    s1_map[s1_parent] = s2_parent;
                }
            }
            if s1_parent != base_parent {
                if s1_parent < base_parent {
                    base_map[base_parent] = s1_parent;
                } else {
                    base_map[s1_parent] = base_parent;
                }
            }
            i += 1;
        }
        i = 0;
        while i < base.len() {
            let base_char = base[i];
            let base_index = (base_char as u8 - 'a' as u8) as usize;
            let base_parent = Solution::find_parent(base_index, &base_map);
            result[i] = base_parent;
            i += 1;
        }
        let mut result_str = String::new();
        i = 0;
        while i < result.len() {
            let result_char = (result[i] as u8 + 'a' as u8) as char;
            result_str.push(result_char);
            i += 1;
        }
        return result_str;
    }
}
