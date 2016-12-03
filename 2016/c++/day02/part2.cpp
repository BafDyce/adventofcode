#include "Day02.hpp"
#include "../AoC.hpp"

Result Day02::solve_p2() {

    string res = "";

    // remember where we point to
    int finger = 5;

    for(unsigned ii = 0; ii < data.size(); ii++){
        string instructions = data[ii];

        // keypad:
        //     1
        //   2 3 4
        // 5 6 7 8 9
        //   A B C
        //     D
        // in the code i'll treat A, B, C, D as 10, 11, 12, 13 respectively

        for(unsigned instr = 0; instr < instructions.length(); instr++){

            switch( instructions[instr] ){
            case 'U':
                // quick 'n dirty "movement table"
                switch( finger ){
                case 1: case 2: case 4: case 5: case 9: break;
                case 6: case 8: case 10: case 12: finger-=4; break;
                case 3: finger-=2; break;
                case 7: finger-=4; break;
                case 11: finger-=4; break;
                case 13: finger-=2; break;
                }
                break;
            case 'D':
                switch( finger ){
                case 5: case 10: case 13: case 12: case 9: break;
                case 2: case 4: case 6: case 8: finger+=4; break;
                case 1: finger+=2; break;
                case 3: finger+=4; break;
                case 7: finger+=4; break;
                case 11: finger+=2; break;
                }
                break;
            case 'R':
                switch( finger ){
                case 1: case 4: case 9: case 12: case 13: break;
                default:
                    finger++;
                    break;
                }
                break;
            case 'L':
                switch( finger ){
                case 1: case 2: case 5: case 10: case 13: break;
                default:
                    finger--;
                    break;
                }
                break;
            default:
                return {false, "Invalid input!"};
            }
        }

        // format & save key
        if( finger > 9 && finger < 14 ){
            res += ('A' + finger - 10);
        } else {
            res += to_string(finger);
        }

    }

    return {true, res};
}
