#include "Day23.hpp"

#include <regex>
#include <vector>
#include <sstream>

#include "../util/Assembunny.hpp"

using namespace std;

// parse_instructions() is implemented in ../util/Assembunny.cpp
Day23::Day23(vector<string> input) : instructions(parse_assembunny(input)) {

}

Day23::~Day23(){
}
