#include <iostream>
#include <vector>
#include <assert.h>
#include <stack>
// Definition for a Node.
class Node {
public:
    int val;
    Node* prev;
    Node* next;
    Node* child;
};


class Solution {
public:
    Node* flatten(Node* head) {
        if (head == nullptr) {
            return nullptr;
        }

        std::stack<Node*> st;
        Node* current = head;

        while (current != nullptr) {
            if (current->child != nullptr) {
                if (current->next != nullptr) {
                    st.push(current->next);
                }

                current->next = current->child;
                current->child->prev = current;
                current->child = nullptr;
            } else if (current->next == nullptr && !st.empty()) {
                Node* next = st.top();
                st.pop();
                current->next = next;
                next->prev = current;
            }

            current = current->next;
        }

        return head;
    }
};

int main(void) {
    Solution s;
    Node* head = nullptr;
    Node* result = s.flatten(head);
    return 0;
}
