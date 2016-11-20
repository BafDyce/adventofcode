#include "Day06.h"

bool Day06::solve_p2(string& result){
    for(unsigned ii = 0; ii < instructions.size(); ii++){
        Instruction instr = instructions[ii];
        switch(instr.action){
        case ON:
            brighten(instr.start, instr.end);
            break;
        case OFF:
            dim(instr.start, instr.end);
            break;
        case TOGGLE:
            brighten(instr.start, instr.end, 2);
            break;
        default:
            result = "Ooops, an error occured!";
            return false;
        }
    }

    result = to_string( count_lights() );
    return true;
}
