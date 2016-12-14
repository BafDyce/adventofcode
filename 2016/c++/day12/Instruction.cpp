#include "Instruction.hpp"

#include <vector>
#include <iostream>

using namespace std;

void Instruction::print(void){
    string typestring;
    switch(type){
    case COPY: typestring = "COPY"; break;
    case INC: typestring = "INC"; break;
    case DEC: typestring = "DEC"; break;
    case JNZ: typestring = "JNZ"; break;
    default: cerr << "SHOULD NEVER HAPPEN (Instruction::print)" << endl;
    }

    cout << typestring << " from " << source
        << " (" << source_is_reg << ")"<< " to " << target << endl;
}

unsigned Instruction::execute(vector<int> &registers){
    switch(type){
    case COPY:
        if( source_is_reg ){
            registers[target] = registers[source];
        } else {
            registers[target] = source;
        }
        break;
    case INC:
        ++registers[source];
        break;
    case DEC:
        --registers[source];
        break;
    case JNZ:
        if( registers[source] != 0 ){
            return target;
        }
        break;
    default: cerr << "SHOULD NEVER HAPPEN (Instruction::execute)" << endl;
    }

    return 1;
}
