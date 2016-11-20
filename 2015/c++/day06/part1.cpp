#include "Day06.h"

bool Day06::solve_p1(string& result){
    for(unsigned ii = 0; ii < instructions.size(); ii++){
        Instruction instr = instructions[ii];
        switch(instr.action){
        case ON:
            turn_on(instr.start, instr.end);
            break;
        case OFF:
            turn_off(instr.start, instr.end);
            break;
        case TOGGLE:
            toggle(instr.start, instr.end);
            break;
        default:
            result = "Ooops, an error occured!";
            return false;
        }
    }

    result = to_string( count_lights() );
    return true;
}
