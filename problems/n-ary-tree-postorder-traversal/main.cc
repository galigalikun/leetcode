
#include <vector>
#include <iostream>
#include <map>
#include <iterator>
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

// https://engineer.yeele.net/algorithm/leetcode-easy-590-n-ary-tree-postorder-traversal/

class Solution {
public:
    std::vector<int> postorder(Node* root) {
        std::vector<int> ans(0);
        if (root == nullptr) {
            return ans;
        }
        std::vector<Node*> stack = {root};
        while (!stack.empty()) {
            auto node = stack.back();
            stack.pop_back();
            ans.push_back(node->val);
            if (!node->children.empty())
            {
                for (auto i : node->children)
                {
                    stack.push_back(i);
                }
            }
        }
        std::reverse(ans.begin(), ans.end());
        return ans;
    }
};

int main() {
    auto s = new Solution();
    auto ans = s->postorder(new Node(1, {
        new Node(3, {
            new Node(5),
            new Node(6)
        }),
        new Node(2),
        new Node(4)
    }));
    std::cout << "[";
    for (const auto &a : ans)
    {
        std::cout << a << ",";
    }
    std::cout << "}" << std::endl;

    return 0;
}
