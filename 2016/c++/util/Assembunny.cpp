#include "Assembunny.hpp"

#include <vector>
#include <iostream>
#include <regex>

using namespace std;

vector<Instruction> parse_assembunny(vector<string> assembunny){
    vector<Instruction> instructions;
    unsigned id_counter = 0;

    regex regSingle(R"((dec|inc|tgl) ([a-d\d\-]+))");
    regex regDouble(R"((cpy|jnz|) ([a-d\d\-]+) ([a-d\d\-]+))");
    for(string line: assembunny){
        smatch m;
        if( std::regex_match(line, m, regSingle) ){
            InstrType type;
            if( m[1] == "inc" ){
                type = INC;
            } else if( m[1] == "dec" ){
                type = DEC;
            } else if( m[1] == "tgl" ){
                type = TGL;
            } else {
                cerr << "invalid single instruction: " << m[0] << endl;
                continue;
            }

            int source = 0;
            bool is_reg = false;
            string tmp = m[2];
            if( tmp[0] >= 'a' && tmp[0] <= 'd' ){
                source = tmp[0] - 'a';
                is_reg = true;
            } else {
                source = stoi(m[2]);
            }

            instructions.push_back({id_counter, type, source, is_reg, 0, false});
        } else if ( std::regex_match(line, m, regDouble) ) {
            InstrType type;
            if( m[1] == "cpy" ){
                type = CPY;
            } else if( m[1] == "jnz" ){
                type = JNZ;
            } else {
                cerr << "invalid double instruction: " << m[0] << endl;
                continue;
            }

            int source = 0;
            bool source_is_reg = false;
            string tmp = m[2];
            if( tmp[0] >= 'a' && tmp[0] <= 'd' ){
                source = tmp[0] - 'a';
                source_is_reg = true;
            } else {
                source = stoi(m[2]);
            }

            int target = 0;
            bool target_is_reg = false;
            string tmp_target = m[3];
            if( tmp_target[0] >= 'a' && tmp_target[0] <= 'd' ){
                target = tmp_target[0] - 'a';
                target_is_reg = true;
            } else {
                target = stoi(m[3]);
            }

            instructions.push_back({id_counter, type,
                source, source_is_reg, target, target_is_reg});
        } else {
            cerr << "oops @ " << line << endl;
        }

        ++id_counter;
    }

    return instructions;
}

void Instruction::print(void){
    string typestring;
    switch(type){
    case CPY: typestring = "CPY"; break;
    case INC: typestring = "INC"; break;
    case DEC: typestring = "DEC"; break;
    case JNZ: typestring = "JNZ"; break;
    case TGL: typestring = "TGL"; break;
    default: cerr << "SHOULD NEVER HAPPEN (Instruction::print)" << endl;
    }

    cout << id << ": " << typestring
        << " from " << source << " (" << source_is_reg << ")"
        << " to " << target << " (" << target_is_reg << ")" << endl;
}

unsigned Instruction::execute(vector<int> &registers){
    switch(type){
    case CPY:
        if( !target_is_reg ){
            break;
        }

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
        if( source_is_reg && registers[source] != 0 ) {
            if( target_is_reg ){
                return registers[target];
            } else {
                return target;
            }
        }
        if( !source_is_reg && source != 0 ){
            if( target_is_reg ){
                return registers[target];
            } else {
                return target;
            }
        }
        break;
    case TGL:
        break;
    default: cerr << "SHOULD NEVER HAPPEN (Instruction::execute)" << endl;
    }

    return 1;
}

unsigned
Instruction::execute(vector<int> &registers, vector<Instruction> &instructions){
    switch(type){
    case CPY: case INC: case DEC: case JNZ:
        return this->execute(registers);
        break;
    case TGL: {
        unsigned idx = this->id;
        if( source_is_reg ){
            idx += registers[source];
        } else {
            idx += source;
        }

        if( idx >= instructions.size() ){
            break;
        }

        Instruction &instr = instructions[idx];

        switch(instr.type){
            // single argument instructions
            case INC: instr.type = DEC; break;
            case DEC: case TGL: instr.type = INC; break;
            // double argument instructions
            case JNZ: instr.type = CPY; break;
            case CPY: instr.type = JNZ; break;
            default:
                cerr << "SHOULD NEVER HAPPEN (Instruction::execute)" << endl;
        }

        break;
    }
    default: cerr << "SHOULD NEVER HAPPEN (Instruction::execute23)" << endl;
    }

    return 1;
}
