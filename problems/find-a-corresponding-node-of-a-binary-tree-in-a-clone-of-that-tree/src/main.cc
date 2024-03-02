#include <iostream>

int main() {
    auto o = new TreeNode(7);
    o->left = new TreeNode(4);
    o->right = new TreeNode(3);
    o->right->left = new TreeNode(6);
    o->right->right = new TreeNode(19);
    auto s = Solution();
    auto t = new TreeNode(3);
    std::cout << s.getTargetCopy(o, o, t) << std::endl;
    return 0;
}

struct Solution;

/**
 Definition for a binary tree node.
  */
struct TreeNode {
     int val;
     TreeNode *left;
     TreeNode *right;
     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};


class Solution {
public:
    TreeNode* getTargetCopy(TreeNode* original, TreeNode* cloned, TreeNode* target) {
        
    }
};
