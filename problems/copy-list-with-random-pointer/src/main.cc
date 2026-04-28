#include <iostream>
#include <assert.h>
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};


class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (head == nullptr) {
            return nullptr;
        }

        // 1) Interleave copied nodes: A->A'->B->B'...
        Node* curr = head;
        while (curr != nullptr) {
            Node* copy = new Node(curr->val);
            copy->next = curr->next;
            curr->next = copy;
            curr = copy->next;
        }

        // 2) Assign random pointers for copied nodes.
        curr = head;
        while (curr != nullptr) {
            if (curr->random != nullptr) {
                curr->next->random = curr->random->next;
            }
            curr = curr->next->next;
        }

        // 3) Detach copied list from original list.
        Node* copiedHead = head->next;
        curr = head;
        while (curr != nullptr) {
            Node* copy = curr->next;
            curr->next = copy->next;
            if (copy->next != nullptr) {
                copy->next = copy->next->next;
            }
            curr = curr->next;
        }

        return copiedHead;
    }
};

int main(void) {
    // Input: head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
    // Output: [[7,null],[13,0],[11,4],[10,2],[1,0]]
    auto node1 = new Node(7);
    auto node2 = new Node(13);
    auto node3 = new Node(11);
    auto node4 = new Node(10);
    auto node5 = new Node(1);

    node1->next = node2;
    node2->next = node3;
    node3->next = node4;
    node4->next = node5;

    node1->random = nullptr;
    node2->random = node1;
    node3->random = node5;
    node4->random = node3;
    node5->random = node1;

    auto solution = new Solution();
    Node* copied = solution->copyRandomList(node1);

    // Value checks
    assert(copied->val == 7);
    assert(copied->next->val == 13);
    assert(copied->next->next->val == 11);
    assert(copied->next->next->next->val == 10);
    assert(copied->next->next->next->next->val == 1);

    // Random checks on copied list (same structure as input/output)
    assert(copied->random == nullptr);
    assert(copied->next->random == copied);
    assert(copied->next->next->random == copied->next->next->next->next);
    assert(copied->next->next->next->random == copied->next->next);
    assert(copied->next->next->next->next->random == copied);

    // Deep-copy checks (must not point into original list)
    assert(copied != node1);
    assert(copied->next != node2);
    assert(copied->next->next != node3);
    assert(copied->next->next->next != node4);
    assert(copied->next->next->next->next != node5);

    return 0;
}
