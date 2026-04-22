
#include <assert.h>

// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val) : val(_val), left(nullptr), right(nullptr), next(nullptr) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};


class Solution {
public:
    Node* connect(Node* root) {
        if (!root) {
            return nullptr;
        }

        Node* leftmost = root;
        while (leftmost->left) {
            Node* current = leftmost;
            while (current) {
                current->left->next = current->right;
                if (current->next) {
                    current->right->next = current->next->left;
                }
                current = current->next;
            }
            leftmost = leftmost->left;
        }

        return root;
    }
};

int main(int argc, char const* argv[]) {
    // Input: root = [1,2,3,4,5,6,7]
    // Output: [1,#,2,3,#,4,5,6,7,#]
    Solution solution;

    assert(solution.connect(nullptr) == nullptr);

    Node* n4 = new Node(4);
    Node* n5 = new Node(5);
    Node* n6 = new Node(6);
    Node* n7 = new Node(7);
    Node* n2 = new Node(2, n4, n5, nullptr);
    Node* n3 = new Node(3, n6, n7, nullptr);
    Node* n1 = new Node(1, n2, n3, nullptr);

    Node* root = solution.connect(n1);
    assert(root == n1);
    assert(n1->next == nullptr);
    assert(n2->next == n3);
    assert(n3->next == nullptr);
    assert(n4->next == n5);
    assert(n5->next == n6);
    assert(n6->next == n7);
    assert(n7->next == nullptr);

    return 0;
}
