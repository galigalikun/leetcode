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
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *a = headA, *b = headB;
        while (a != b) {
            a = a ? a->next : headB;
            b = b ? b->next : headA;
        }
        return a;
    }
};

int main() {
    // Input: intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA = 2, skipB = 3
    // Output: Intersected at '8'
    // Shared tail: [8,4,5]
    ListNode *n8 = new ListNode(8);
    ListNode *n4b = new ListNode(4);
    ListNode *n5 = new ListNode(5);
    n8->next = n4b;
    n4b->next = n5;

    // listA: 4 -> 1 -> 8 -> 4 -> 5
    ListNode *a1 = new ListNode(4);
    ListNode *a2 = new ListNode(1);
    a1->next = a2;
    a2->next = n8;

    // listB: 5 -> 6 -> 1 -> 8 -> 4 -> 5
    ListNode *b1 = new ListNode(5);
    ListNode *b2 = new ListNode(6);
    ListNode *b3 = new ListNode(1);
    b1->next = b2;
    b2->next = b3;
    b3->next = n8;

    Solution sol;
    ListNode *result = sol.getIntersectionNode(a1, b1);
    assert(result != nullptr && result->val == 8);
    std::cout << "Intersected at '" << result->val << "'" << std::endl;
    return 0;
}
