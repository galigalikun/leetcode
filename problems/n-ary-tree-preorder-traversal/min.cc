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
    std::vector<int> preorder(Node* root) {
        std::vector<int> a(1);
        return a;
    }
};

int main() {
    auto s = new Solution();
    auto ans = s->preorder(new Node(1, {
        new Node(3, {
            new Node(5),
            new Node(4)
        }),
        new Node(2),
        new Node(4)
    }));
    // std::cout << "ans->" << ans << std::endl;
    return 0;
}
