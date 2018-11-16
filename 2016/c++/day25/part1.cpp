#include "Day25.hpp"

#include <vector>
#include <iostream>

#include "../util/Assembunny.hpp"

using namespace std;

Result Day25::solve_p1(){
    // parsing of input happens in ../util/Assembunny.cpp
    vector<int> registers = {0, 0, 0, 0};


    unsigned value_a = 0;
    while ( true ) {
        registers[0] = value_a;
        registers[1] = 0;
        registers[2] = 0;
        registers[3] = 0;
        unsigned next = 0;
        int last_outvalue = -1;
        int outvalue_check_counter = 0;
        //cout << "value a: " << value_a << endl;
        while ( next >= 0 && next < instructions.size() ) {

            int outvalue = -1;
            int outvalue_check = instructions[next].type == OUT;
            next += instructions[next].execute(registers, instructions, &outvalue);
            if ( outvalue_check ) {
                //cout << "outvalue: " << outvalue << endl;
                if ( outvalue == -1 ) {
                    cerr << "Bad error" << endl;
                } else if ( outvalue == last_outvalue ) {
                    break;
                } else if ( outvalue_check_counter > 1000000 ) {
                    return {true, to_string(value_a)};
                } else {
                    last_outvalue = outvalue;
                    outvalue_check_counter += 1;
                }
            }
        }

        value_a += 1;
    }


    return {true, to_string(registers[0])};
}
