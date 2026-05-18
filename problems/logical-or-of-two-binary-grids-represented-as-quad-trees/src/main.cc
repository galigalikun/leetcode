#include <assert.h>
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
        topLeft = nullptr;
        topRight = nullptr;
        bottomLeft = nullptr;
        bottomRight = nullptr;
    }

    Node(bool _val, bool _isLeaf) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = nullptr;
        topRight = nullptr;
        bottomLeft = nullptr;
        bottomRight = nullptr;
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
    Node* intersect(Node* quadTree1, Node* quadTree2) {
        if (quadTree1->isLeaf) {
            if (quadTree1->val) {
                return new Node(true, true);
            }
            return clone(quadTree2);
        }

        if (quadTree2->isLeaf) {
            if (quadTree2->val) {
                return new Node(true, true);
            }
            return clone(quadTree1);
        }

        Node* tl = intersect(quadTree1->topLeft, quadTree2->topLeft);
        Node* tr = intersect(quadTree1->topRight, quadTree2->topRight);
        Node* bl = intersect(quadTree1->bottomLeft, quadTree2->bottomLeft);
        Node* br = intersect(quadTree1->bottomRight, quadTree2->bottomRight);

        if (tl->isLeaf && tr->isLeaf && bl->isLeaf && br->isLeaf &&
            tl->val == tr->val && tr->val == bl->val && bl->val == br->val) {
            return new Node(tl->val, true);
        }

        return new Node(false, false, tl, tr, bl, br);
    }

private:
    Node* clone(Node* node) {
        if (node->isLeaf) {
            return new Node(node->val, true);
        }

        return new Node(
            false,
            false,
            clone(node->topLeft),
            clone(node->topRight),
            clone(node->bottomLeft),
            clone(node->bottomRight)
        );
    }
};

int main() {
    assert(Solution().intersect(
        new Node(false, false,
            new Node(true, true),
            new Node(true, true),
            new Node(false, true),
            new Node(false, true)
        ),
        new Node(false, false,
            new Node(true, true),
            new Node(false, true),
            new Node(true, true),
            new Node(false, true)
        )
    )->isLeaf == true);
    return 0;
}
