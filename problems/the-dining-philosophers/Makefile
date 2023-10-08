CXXFLAGS = -std=c++17
TARGET = solution
SOURCE = src/main.cc

all: $(TARGET)

$(TARGET): $(SOURCE)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SOURCE)

.PHONY: clean

clean:
	rm -f $(TARGET)
