fn main() {
    assert_eq!(Solution::largest_component_size(vec![4,6,15,35]), 4);
    assert_eq!(Solution::largest_component_size(vec![20,50,9,63]), 2);
    assert_eq!(Solution::largest_component_size(vec![2,3,6,7,4,12,21,39]), 8);
}

struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        let size = vec![1; n];
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
    }

    fn component_size(&mut self, x: usize) -> i32 {
        let root = self.find(x);
        self.size[root]
    }
}

struct Solution;
impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dsu = DisjointSetUnion::new(n);
        let mut factor_owner = std::collections::HashMap::<i32, usize>::new();

        for (i, &num) in nums.iter().enumerate() {
            let mut value = num;
            let mut divisor = 2;

            while divisor * divisor <= value {
                if value % divisor == 0 {
                    if let Some(&j) = factor_owner.get(&divisor) {
                        dsu.union(i, j);
                    } else {
                        factor_owner.insert(divisor, i);
                    }

                    while value % divisor == 0 {
                        value /= divisor;
                    }
                }
                divisor += 1;
            }

            if value > 1 {
                if let Some(&j) = factor_owner.get(&value) {
                    dsu.union(i, j);
                } else {
                    factor_owner.insert(value, i);
                }
            }
        }

        let mut best = 0;
        for i in 0..n {
            best = best.max(dsu.component_size(i));
        }

        best
    }
}
