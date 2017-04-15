#include "Day12.hpp"

#include <vector>
#include <iostream>
#include <sstream>

#include "../util/Assembunny.hpp"

using namespace std;

Result Day12::solve_p1(){
    // parsing of input happens in ../util/Assembunny.cpp
    vector<int> registers = {0, 0, 0, 0};

    unsigned next = 0;
    while( next < instructions.size() ){
        // execute is implemented in ../util/Assembunny.cpp
        next += instructions[next].execute(registers);
    }

    return {true, to_string(registers[0])};
}
