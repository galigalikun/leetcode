#include <iostream>
#include <assert.h>
// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};
class Solution {
public:
    void deleteNode(ListNode* node) {
        node->val = node->next->val;
        node->next = node->next->next;
    }
};

int main() {
    Solution s;
    ListNode* head = new ListNode(4);
    head->next = new ListNode(5);
    head->next->next = new ListNode(1);
    head->next->next->next = new ListNode(9);
    s.deleteNode(head->next); // delete node with value 5

    // Expected: 4 -> 1 -> 9
    assert(head->val == 4);
    assert(head->next->val == 1);
    assert(head->next->next->val == 9);
    assert(head->next->next->next == nullptr);

    return 0;
}
