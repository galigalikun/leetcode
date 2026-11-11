#include <functional>
#include <condition_variable>
#include <mutex>
#include <thread>
#include <vector>
#include <cstdio>

using namespace std;

class H2O {
private:
    mutex lock;
    condition_variable cv;
    int h_count = 0;
    int o_count = 0;

public:
    H2O() {}

    void hydrogen(function<void()> releaseHydrogen) {
        {
            unique_lock<mutex> lk(lock);
            // Wait until there are fewer than 2 hydrogens and no oxygen assigned yet
            cv.wait(lk, [this] { return h_count < 2 && o_count == 0; });
            h_count++;
            
            // When 2nd hydrogen arrives, notify oxygen
            if (h_count == 2) {
                cv.notify_all();
            }
            
            // Wait for oxygen to join
            cv.wait(lk, [this] { return o_count == 1; });
        }
        
        releaseHydrogen();
        
        {
            unique_lock<mutex> lk(lock);
            h_count--;
            if (h_count == 0) {
                o_count = 0;
                cv.notify_all();
            }
        }
    }

    void oxygen(function<void()> releaseOxygen) {
        {
            unique_lock<mutex> lk(lock);
            // Wait for exactly 2 hydrogens and no oxygen assigned
            cv.wait(lk, [this] { return h_count == 2 && o_count == 0; });
            o_count = 1;
            cv.notify_all();  // Notify both hydrogens
        }
        
        releaseOxygen();
        
        {
            unique_lock<mutex> lk(lock);
            // Wait for hydrogens to finish and reset
            cv.wait(lk, [this] { return o_count == 0; });
        }
    }
};

int main() {
    auto h2o = new H2O();
    
    vector<thread> threads;
    
    // First molecule: H H O
    threads.push_back(thread([h2o](){ h2o->hydrogen([](){ printf("H"); }); }));
    threads.push_back(thread([h2o](){ h2o->hydrogen([](){ printf("H"); }); }));
    threads.push_back(thread([h2o](){ h2o->oxygen([](){ printf("O"); }); }));
    
    // Second molecule: H H O
    threads.push_back(thread([h2o](){ h2o->hydrogen([](){ printf("H"); }); }));
    threads.push_back(thread([h2o](){ h2o->hydrogen([](){ printf("H"); }); }));
    threads.push_back(thread([h2o](){ h2o->oxygen([](){ printf("O"); }); }));
    
    for (auto& t : threads) {
        t.join();
    }
    
    printf("\n");
    fflush(stdout);
    delete h2o;
    return 0;
}

