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
    std::vector<int> ans;
    void helper(Node *root)
    {
        if (root != nullptr) {
            ans.push_back(root->val);
            for (const auto &c : root->children)
            {
                this->helper(c);
            }
        }
    }

public:
    Solution() :ans(0) {}
    std::vector<int> preorder(Node* root) {
        this->helper(root);
        return ans;
    }
};

int main() {
    auto s = new Solution();
    auto ans = s->preorder(new Node(1, {
        new Node(3, {
            new Node(5),
            new Node(6)
        }),
        new Node(2),
        new Node(4)
    }));
    std::cout << "[";
    for (auto a : ans)
    {
        std::cout << a << ",";
    }
    std::cout << "]" << std::endl;
    return 0;
}
