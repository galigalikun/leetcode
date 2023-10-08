#include <functional>
#include <iostream>

using namespace std;

class DiningPhilosophers {
public:
    DiningPhilosophers() {
        
    }

    void wantsToEat(int philosopher,
                    function<void()> pickLeftFork,
                    function<void()> pickRightFork,
                    function<void()> eat,
                    function<void()> putLeftFork,
                    function<void()> putRightFork) {
                        pickLeftFork = [philosopher, pickLeftFork]() {
                            cout << "Philosopher " << philosopher << " picks left fork" << endl;
                            pickLeftFork();
                        };
                        pickRightFork = [philosopher, pickRightFork]() {
                            cout << "Philosopher " << philosopher << " picks right fork" << endl;
                            pickRightFork();
                        };
                        eat = [philosopher, eat]() {
                            cout << "Philosopher " << philosopher << " eats" << endl;
                            eat();
                        };
                        putLeftFork = [philosopher, putLeftFork]() {
                            cout << "Philosopher " << philosopher << " puts left fork" << endl;
                            putLeftFork();
                        };
                        putRightFork = [philosopher, putRightFork]() {
                            cout << "Philosopher " << philosopher << " puts right fork" << endl;
                            putRightFork();
                        };
                        pickLeftFork();
                        pickRightFork();
                        eat();
                        putLeftFork();
                        putRightFork();
    }
};

int main(void) {
    DiningPhilosophers dp;
    dp.wantsToEat(0, []() {}, []() {}, []() {}, []() {}, []() {});
    dp.wantsToEat(1, []() {}, []() {}, []() {}, []() {}, []() {});
    return 0;
}
