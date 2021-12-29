#include <vector>
#include <iostream>

// Definition for a Node.
class Node {
public:
    int val;
    std::vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, std::vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};


class Solution {
public:
    int maxDepth(Node* root) {
        return 0;
    }
};

int main() {
    Solution* s = new Solution();
    std::vector<Node*> v{
        new Node(3)
    };
    int ans = s->maxDepth(new Node(1, v));
    if (ans != 3) {
        std::cout << "ng" << std::endl;
    }
    return 0;
}
