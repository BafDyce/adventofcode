#include "Day13.hpp"

#include <sstream>
#include <vector>
#include <queue>

Day13::Day13(vector<string> input) : designers_number(0), graph{} {
    stringstream stream(input[0]);
    stream >> this->designers_number;
    this->build_graph();
}

Day13::~Day13(){
}

bool Day13::is_open_space(const int x, const int y){
    if( x < 0 || y < 0 ){
        return false;
    }

    unsigned res = x*x + 3*x + 2*x*y + y + y*y;
    res += this->designers_number;

    int bits = 0;
    for(unsigned ii = 0x80000000; ii != 0; ii >>= 1){
        if( (res & ii) == ii ){
            ++bits;
        }
    }

    return (bits % 2) == 0;
}

void Day13::build_graph(){
    for(unsigned xx = 0; xx < max_x; ++xx){
        for(unsigned yy = 0; yy < max_y; ++yy){
            graph[xx][yy] = is_open_space(xx, yy);
        }
    }
}


