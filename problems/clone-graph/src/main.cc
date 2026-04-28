#include <iostream>
#include <vector>
#include <unordered_map>
#include <assert.h>
// Definition for a Node.
class Node {
public:
    int val;
    std::vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = std::vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = std::vector<Node*>();
    }
    Node(int _val, std::vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};


class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (!node) return nullptr;
        std::unordered_map<Node*, Node*> visited;
        return dfs(node, visited);
    }
private:
    Node* dfs(Node* node, std::unordered_map<Node*, Node*>& visited) {
        if (visited.count(node)) return visited[node];
        Node* clone = new Node(node->val);
        visited[node] = clone;
        for (Node* neighbor : node->neighbors) {
            clone->neighbors.push_back(dfs(neighbor, visited));
        }
        return clone;
    }
};

int main() {
    // Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
    // Output: [[2,4],[1,3],[2,4],[1,3]]
    assert(Solution().cloneGraph(nullptr) == nullptr);

    // Build graph: 1--2--3--4--1, 1--4, 2--3
    Node* n1 = new Node(1);
    Node* n2 = new Node(2);
    Node* n3 = new Node(3);
    Node* n4 = new Node(4);
    n1->neighbors = {n2, n4};
    n2->neighbors = {n1, n3};
    n3->neighbors = {n2, n4};
    n4->neighbors = {n1, n3};

    Node* cloned = Solution().cloneGraph(n1);
    assert(cloned != n1);
    assert(cloned->val == 1);
    assert(cloned->neighbors.size() == 2);
    assert(cloned->neighbors[0]->val == 2);
    assert(cloned->neighbors[1]->val == 4);
    assert(cloned->neighbors[0]->neighbors[0] == cloned);
    assert(cloned->neighbors[0]->neighbors[1]->val == 3);
    assert(cloned->neighbors[1]->neighbors[0] == cloned);
    assert(cloned->neighbors[1]->neighbors[1]->val == 3);

    return 0;
}
