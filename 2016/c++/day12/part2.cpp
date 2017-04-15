#include "Day12.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

#include "../util/Assembunny.hpp"

using namespace std;

Result Day12::solve_p2(){
    // parsing of input happens in ../util/Assembunny.cpp
    vector<int> registers = {0, 0, 1, 0};

    unsigned next = 0;
    while( next < instructions.size() ){
        // execute is implemented in ../util/Assembunny.cpp
        next += instructions[next].execute(registers);
    }

    return {true, to_string(registers[0])};
}
