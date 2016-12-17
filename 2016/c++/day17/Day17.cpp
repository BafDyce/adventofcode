#include "Day17.hpp"

Day17::Day17(vector<string> input) : data(input[0]) {
}

Day17::~Day17() {
}

bool Day17::is_open(char ch){
    switch(ch){
    case 'b': case 'c': case 'd': case 'e': case 'f': return true;
    default: return false;
    }
}
