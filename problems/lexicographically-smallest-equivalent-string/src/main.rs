fn main() {
    assert_eq!(
        Solution::smallest_equivalent_string(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string(),
        ),
        "makkek"
    );
    assert_eq!(
        Solution::smallest_equivalent_string(
            "hello".to_string(),
            "world".to_string(),
            "hold".to_string(),
        ),
        "hdld"
    );
    assert_eq!(
        Solution::smallest_equivalent_string(
            "leetcode".to_string(),
            "programs".to_string(),
            "sourcecode".to_string(),
        ),
        "aauaaaaada"
    );
}

struct Solution;

impl Solution {
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);

        if root_x == root_y {
            return;
        }

        if rank[root_x] < rank[root_y] {
            parent[root_x] = root_y;
        } else if rank[root_x] > rank[root_y] {
            parent[root_y] = root_x;
        } else {
            parent[root_y] = root_x;
            rank[root_x] += 1;
        }
    }

    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut parent: Vec<usize> = (0..26).collect();
        let mut rank = vec![0; 26];

        for (a, b) in s1.chars().zip(s2.chars()) {
            let x = (a as u8 - b'a') as usize;
            let y = (b as u8 - b'a') as usize;
            Self::union(&mut parent, &mut rank, x, y);
        }

        let mut smallest = vec!['z'; 26];
        for ch in 0..26 {
            let root = Self::find(&mut parent, ch);
            let letter = (b'a' + ch as u8) as char;
            if letter < smallest[root] {
                smallest[root] = letter;
            }
        }

        let mut result = String::with_capacity(base_str.len());
        for ch in base_str.chars() {
            let index = (ch as u8 - b'a') as usize;
            let root = Self::find(&mut parent, index);
            result.push(smallest[root]);
        }

        result
    }
}
