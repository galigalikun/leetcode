#include <vector>
using namespace std;
#include <assert.h>
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};


class Solution {
public:
    vector<vector<int>> levelOrder(Node* root) {
        vector<vector<int>> result;
        if (!root) return result;
        vector<Node*> currentLevel = {root};
        while (!currentLevel.empty()) {
            vector<int> currentValues;
            vector<Node*> nextLevel;
            for (auto node : currentLevel) {
                currentValues.push_back(node->val);
                for (auto child : node->children) {
                    nextLevel.push_back(child);
                }
            }
            result.push_back(currentValues);
            currentLevel = nextLevel;
        }
        return result;
    }
};

int main() {
    auto n1 = new Node(1);
    auto n2 = new Node(2);
    auto n3 = new Node(3);
    auto n4 = new Node(4);
    auto n5 = new Node(5);
    auto n6 = new Node(6);
    n1->children = {n3, n2, n4};
    n3->children = {n5, n6};
    auto res = (new Solution())->levelOrder(n1);
    assert(res.size() == 3);
    assert(res[0].size() == 1);
    assert(res[0][0] == 1);
    assert(res[1].size() == 3);
    assert(res[1][0] == 3);
    assert(res[1][1] == 2);
    assert(res[1][2] == 4);
    assert(res[2].size() == 2);
    assert(res[2][0] == 5);
    assert(res[2][1] == 6);
    return 0;
}
