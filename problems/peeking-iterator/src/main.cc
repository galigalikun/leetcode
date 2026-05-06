#include <iostream>
#include <vector>
#include <assert.h>
using std::vector;
// Below is the interface for Iterator, which is already defined for you.
// **DO NOT** modify the interface for Iterator.

class Iterator {
    struct Data;
    Data* data;
public:
    Iterator(const vector<int>& nums);
    Iterator(const Iterator& iter);

    // Returns the next element in the iteration.
    int next();

    // Returns true if the iteration has more elements.
    bool hasNext() const;
};

struct Iterator::Data {
	vector<int> nums;
	size_t index;
};

Iterator::Iterator(const vector<int>& nums) {
	data = new Data{nums, 0};
}

Iterator::Iterator(const Iterator& iter) {
	data = new Data{iter.data->nums, iter.data->index};
}

int Iterator::next() {
	return data->nums[data->index++];
}

bool Iterator::hasNext() const {
	return data->index < data->nums.size();
}


class PeekingIterator : public Iterator {
private:
    bool has_peeked;
    int peeked_value;

public:
	PeekingIterator(const vector<int>& nums) : Iterator(nums), has_peeked(false), peeked_value(0) {
	    // Initialize any member here.
	    // **DO NOT** save a copy of nums and manipulate it directly.
	    // You should only use the Iterator interface methods.

	}

    // Returns the next element in the iteration without advancing the iterator.
	int peek() {
	    if (!has_peeked) {
	        peeked_value = Iterator::next();
	        has_peeked = true;
	    }
	    return peeked_value;
	}

	// hasNext() and next() should behave the same as in the Iterator interface.
	// Override them if needed.
	int next() {
	    if (has_peeked) {
	        has_peeked = false;
	        return peeked_value;
	    }
	    return Iterator::next();
	}

	bool hasNext() const {
	    return has_peeked || Iterator::hasNext();
	}
};

int main(void) {
    auto nums = vector<int>{1, 2, 3};
    auto iter = PeekingIterator(nums);
    assert(iter.next() == 1);  // Returns 1 without advancing the iterator.
    assert(iter.peek() == 2);  // Returns 2 without advancing the iterator.
    assert(iter.next() == 2);  // Returns 2 and advances the iterator to the next element.
    assert(iter.next() == 3);  // Returns 3 without advancing the iterator.
    assert(iter.hasNext() == false); // Returns true because there is still one element in the iterator.
    return 0;
}
