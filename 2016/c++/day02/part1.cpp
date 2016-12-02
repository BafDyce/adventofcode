#include "Day02.hpp"
#include "../AoC.hpp"

Result Day02::solve_p1() {

    string res = "";

    // remember where we point to
    int finger = 5;

    for(unsigned ii = 0; ii < data.size(); ii++){
        string instructions = data[ii];

        for(unsigned instr = 0; instr < instructions.length(); instr++){

            // just move along the keyboard
            switch( instructions[instr] ){
            case 'U':
                // "If a move doesn't lead to a button, ignore it."
                if( finger > 3 ){
                    finger -= 3;
                }
                break;
            case 'D':
                if( finger < 7 ){
                    finger += 3;
                }
                break;
            case 'R':
                if ( finger % 3 != 0 ){
                    finger++;
                }
                break;
            case 'L':
                if( finger % 3 != 1 ){
                    finger--;
                }
                break;
            default:
                return {false, "Invalid input!"};
            }
        }

        // at the end of one instruction line, save where we are right now
        res += to_string(finger);

    }

    return {true, res};
}
