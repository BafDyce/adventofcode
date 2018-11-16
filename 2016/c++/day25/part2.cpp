#include "Day25.hpp"

#include <vector>
#include <iostream>

#include "../util/Assembunny.hpp"

using namespace std;

Result Day25::solve_p2(){
    // parsing of input happens in ../util/Assembunny.cpp
    vector<int> registers = {12, 0, 0, 0};

    unsigned next = 0;
    while( next < instructions.size() ){
        next += instructions[next].execute(registers, instructions);

        if( next < 0 ){
            cerr << "oO next is " << next << " | a is " << registers[0] << endl;
            next = 0;
        }
    }

    return {true, to_string(registers[0])};
}
