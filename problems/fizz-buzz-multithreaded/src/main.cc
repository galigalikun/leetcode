#include <iostream>
#include <thread>
#include <functional>

using namespace std;

int main() {
    auto f = new FizzBuzz(15);
    thread t1(&FizzBuzz::fizz, f, [](){cout << "fizz" << endl;});
    thread t2(&FizzBuzz::buzz, f, [](){cout << "buzz" << endl;});
    thread t3(&FizzBuzz::fizzbuzz, f, [](){cout << "fizzbuzz" << endl;});
    thread t4(&FizzBuzz::number, f, [](int i){cout << i << endl;});
    t1.join();
    t2.join();
    t3.join();
    t4.join();
    
    return 0;
}

class FizzBuzz {
private:
    int n;

public:
    FizzBuzz(int n) {
        this->n = n;
    }

    // printFizz() outputs "fizz".
    void fizz(function<void()> printFizz) {
        
    }

    // printBuzz() outputs "buzz".
    void buzz(function<void()> printBuzz) {
        
    }

    // printFizzBuzz() outputs "fizzbuzz".
	void fizzbuzz(function<void()> printFizzBuzz) {
        
    }

    // printNumber(x) outputs "x", where x is an integer.
    void number(function<void(int)> printNumber) {
        
    }
};
