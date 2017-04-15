#include "Day22.hpp"

#include <regex>

using namespace std;

Day22::Day22(vector<string> input) {

    for(unsigned ii = 2; ii < input.size(); ++ii){
        string &line = input[ii];
        smatch sm;
        regex reg(R"(/dev/grid/node-x([0-9]+)-y([0-9]+)\s+([0-9]+)T\s+([0-9]+)T (.*))");
        if( std::regex_match(line, sm, reg) ){
            unsigned x = stoi(sm[1]);
            unsigned y = stoi(sm[2]);

            grid[x][y].size = stoi(sm[3]);
            grid[x][y].used = stoi(sm[4]);
        }
    }
}

Day22::~Day22(){
}
