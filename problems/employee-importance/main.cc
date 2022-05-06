#include <vector>
#include <iostream>

using namespace std;

// Definition for Employee.
class Employee {
public:
    int id;
    int importance;
    vector<int> subordinates;
};

class Solution {
public:
    int getImportance(vector<Employee*> employees, int id) {
        int sum = 0;
        for (auto e : employees) {
            if (e->id == id) {
                sum += e->importance;
                for (auto s : e->subordinates) {
                    sum += getImportance(employees, s);
                }
            }
        }
        return sum;
    }
};

int main() {
    Solution sol;

    cout << sol.getImportance({
        new Employee{1, 5, {2, 3}},
        new Employee{2, 3, {}},
        new Employee{3, 3, {}}
    }, 1) << endl;

    cout << sol.getImportance({
        new Employee{1, 2, {5}},
        new Employee{5, -3, {}}
    }, 1) << endl;
    return 0;
}
