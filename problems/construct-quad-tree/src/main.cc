
#include <iostream>
#include <vector>
#include <assert.h>
using namespace std;
// Definition for a QuadTree node.
class Node {
public:
    bool val;
    bool isLeaf;
    Node* topLeft;
    Node* topRight;
    Node* bottomLeft;
    Node* bottomRight;

    Node() {
        val = false;
        isLeaf = false;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};

class Solution {
public:
    Node* build(const vector<vector<int>>& grid, int row, int col, int size) {
        int first = grid[row][col];
        bool same = true;

        for (int r = row; r < row + size && same; ++r) {
            for (int c = col; c < col + size; ++c) {
                if (grid[r][c] != first) {
                    same = false;
                    break;
                }
            }
        }

        if (same) {
            return new Node(first == 1, true);
        }

        int half = size / 2;
        Node* topLeft = build(grid, row, col, half);
        Node* topRight = build(grid, row, col + half, half);
        Node* bottomLeft = build(grid, row + half, col, half);
        Node* bottomRight = build(grid, row + half, col + half, half);

        // For internal nodes, val can be either true or false on LeetCode.
        return new Node(true, false, topLeft, topRight, bottomLeft, bottomRight);
    }

    Node* construct(vector<vector<int>>& grid) {
        return build(grid, 0, 0, static_cast<int>(grid.size()));
    }
};

int main(void) {
    Solution solution;
    vector<vector<int>> grid{{0, 1}, {1, 0}};
    Node* root = solution.construct(grid);

    assert(root != nullptr);
    assert(root->isLeaf == false);
    assert(root->topLeft->isLeaf == true && root->topLeft->val == false);
    assert(root->topRight->isLeaf == true && root->topRight->val == true);
    assert(root->bottomLeft->isLeaf == true && root->bottomLeft->val == true);
    assert(root->bottomRight->isLeaf == true && root->bottomRight->val == false);

    return 0;
}
