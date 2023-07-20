#include <functional>

using namespace std;

class FooBar {
private:
    int n;

public:
    FooBar(int n) {
        this->n = n;
    }

    void foo(function<void()> printFoo) {
        
        for (int i = 0; i < n; i++) {
            
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
        }
    }

    void bar(function<void()> printBar) {
        
        for (int i = 0; i < n; i++) {
            
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
        }
    }
};

int main(void) {
    auto f = new FooBar(1);
    f->foo([]() { printf("foo"); });
    f->bar([]() { printf("bar"); });
    return 0;
}
