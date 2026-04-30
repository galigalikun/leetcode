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
    ListNode *detectCycle(ListNode *head) {
        ListNode *slow = head;
        ListNode *fast = head;

        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;

            if (slow == fast) {
                ListNode *entry = head;
                while (entry != slow) {
                    entry = entry->next;
                    slow = slow->next;
                }
                return entry;
            }
        }

        return nullptr;
    }
};

int main() {
    // Input: head = [3,2,0,-4], pos = 1
    // Output: tail connects to node index 1
    // Explanation: There is a cycle in the linked list, where tail connects to the second node.
    auto *head = new ListNode(3);
    head->next = new ListNode(2);
    head->next->next = new ListNode(0);
    head->next->next->next = new ListNode(-4);
    head->next->next->next->next = head->next; // Create a cycle

    Solution sol;
    ListNode *entry = sol.detectCycle(head);

    assert(entry == head->next);
    std::cout << "tail connects to node index 1" << std::endl;
    return 0;
}
