#include "Day12.hpp"

#include <vector>
#include <sstream>

Day12::Day12(vector<string> input) : instructions({}) {
    for(string line: input){
        string type;
        string a;
        string b;

        stringstream stream(line);
        stream >> type >> a >> b;

        if( type == "inc" ){
            instructions.push_back(Instruction{INC, a[0] - 'a', true, 0});
        } else if( type == "dec" ){
            instructions.push_back(Instruction{DEC, a[0] - 'a', true, 0});
        } else if( type == "jnz" ){
            int value;
            stringstream tmp2(b);
            tmp2 >> value;
            instructions.push_back(Instruction{JNZ, a[0] - 'a', true, value});
        } else if( type == "cpy" ){
            if( a[0] >= '0' && a[0] <= '9' ){
                stringstream tmp(a);
                int value;
                tmp >> value;
                instructions.push_back(
                    Instruction{COPY, value, false, b[0] - 'a'});
            } else {
                instructions.push_back(
                    Instruction{COPY, a[0]-'a', true, b[0] - 'a'});
            }
        }
    }
}

Day12::~Day12(){
}
