#include <iostream>
#include <vector>
#include <assert.h>
// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};
class Solution {
public:
    bool hasCycle(ListNode *head) {
        ListNode* slow = head;
        ListNode* fast = head;

        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
            if (slow == fast) {
                return true;
            }
        }

        return false;
    }
};

int main() {
    // Input: head = [3,2,0,-4], pos = 1
    // Output: true
    auto node1 = new ListNode(3);
    auto node2 = new ListNode(2);
    auto node3 = new ListNode(0);
    auto node4 = new ListNode(-4);
    node1->next = node2;
    node2->next = node3;
    node3->next = node4;
    node4->next = node2; // Create a cycle
    assert(Solution().hasCycle(node1) == true);
    return 0;
}
