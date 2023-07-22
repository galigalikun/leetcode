#include <functional>

using namespace std;


class H2O {
public:
    H2O() {
        
    }

    void hydrogen(function<void()> releaseHydrogen) {
        
        // releaseHydrogen() outputs "H". Do not change or remove this line.
        releaseHydrogen();
    }

    void oxygen(function<void()> releaseOxygen) {
        
        // releaseOxygen() outputs "O". Do not change or remove this line.
        releaseOxygen();
    }
};

int main() {
    auto h2o = new H2O();
    h2o->hydrogen([](){ printf("H"); });
    h2o->hydrogen([](){ printf("H"); });
    h2o->oxygen([](){ printf("O"); });
    h2o->hydrogen([](){ printf("H"); });
    h2o->hydrogen([](){ printf("H"); });
    h2o->oxygen([](){ printf("O"); });
    
    return 0;
}
