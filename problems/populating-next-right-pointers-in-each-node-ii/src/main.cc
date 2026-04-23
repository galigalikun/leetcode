#include <iostream>
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
        if (!root) return nullptr;

        Node* curr = root;
        while (curr) {
            Node dummy(0);
            Node* tail = &dummy;

            while (curr) {
                if (curr->left) {
                    tail->next = curr->left;
                    tail = tail->next;
                }
                if (curr->right) {
                    tail->next = curr->right;
                    tail = tail->next;
                }
                curr = curr->next;
            }

            curr = dummy.next;
        }

        return root;
    }
};


int main() {
    auto solution = new Solution();

    // Build tree: [1,2,3,4,5,null,7]
    Node* n4 = new Node(4);
    Node* n5 = new Node(5);
    Node* n7 = new Node(7);
    Node* n2 = new Node(2, n4, n5, nullptr);
    Node* n3 = new Node(3, nullptr, n7, nullptr);
    Node* root = new Node(1, n2, n3, nullptr);

    solution->connect(root);

    // Verify next pointers: 1->#, 2->3->#, 4->5->7->#
    assert(root->next == nullptr);
    assert(n2->next == n3);
    assert(n3->next == nullptr);
    assert(n4->next == n5);
    assert(n5->next == n7);
    assert(n7->next == nullptr);

    return 0;
}
