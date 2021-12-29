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
    int ans;
    void helper(int depth, Node *root)
    {
        if (root != nullptr) {
            ans = std::max(ans, depth);
            for (const auto &c : root->children)
            {
                this->helper(depth+1, c);
            }
        }
    }
public:
    Solution() : ans(0) {}
    int maxDepth(Node* root) {
        this->helper(1, root);
        return ans;
    }
};

int main() {
    auto s = new Solution();
    auto ans = s->maxDepth(new Node(1, {
        new Node(3, {
            new Node(5),
            new Node(6)
        }),
        new Node(2),
        new Node(4)
    }));
    std::cout << "ans->" << ans << std::endl;
    return 0;
}
