#include "Day25.hpp"

#include <regex>
#include <vector>
#include <sstream>

#include "../util/Assembunny.hpp"

using namespace std;

// parse_instructions() is implemented in ../util/Assembunny.cpp
Day25::Day25(vector<string> input) : instructions(parse_assembunny(input)) {

}

Day25::~Day25(){
}
