#include "Day00.hpp"

#include <regex>

using namespace std;

Day00::Day00(vector<string> input) : data(input) {
    for(string &line: input){
        smatch sm;
        regex reg(R"((.*))");
        if( std::regex_match(line, sm, reg) ){
            data.push_back(sm[1]);
        }
    }

}

Day00::~Day00(){
}
