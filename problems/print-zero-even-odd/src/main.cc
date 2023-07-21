#include <functional>

using std::function;

int main(void) {
    auto printNumber = [](int x) { printf("%d", x); };
    ZeroEvenOdd zeo(10);
    zeo.zero(printNumber);
    return 0;
}

class ZeroEvenOdd {
private:
    int n;

public:
    ZeroEvenOdd(int n) {
        this->n = n;
    }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        auto printZero = [&printNumber](int x) { printNumber(0); };
        printZero(0);
    }

    void even(function<void(int)> printNumber) {
        auto printEven = [&printNumber](int x) { printNumber(x); };
        printEven(0);
    }

    void odd(function<void(int)> printNumber) {
        auto printOdd = [&printNumber](int x) { printNumber(x); };
        printOdd(0);
    }
};
