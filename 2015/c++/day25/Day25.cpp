#include "Day25.h"

#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

Day25::Day25() {
}

Day25::Day25(vector<string> input){
    data = input[0];
    // This should be done better..
    stringstream( data.substr(80) ) >> row;
    stringstream( data.substr(92) ) >> col;
}

Day25::~Day25(){
}

unsigned Day25::calc_consecutive_sum_from_1_to(const unsigned n){

    if( n <= 1){
        return n;
    }

    return (n * (n + 1)) / 2;
}

unsigned Day25::calc_position(const unsigned row, const unsigned col){

    // see foto `2015/notes/25/pos-calculation.jpg` for an explanation
    unsigned sum_left = calc_consecutive_sum_from_1_to(col);
    unsigned sum_top = calc_consecutive_sum_from_1_to(row - 1);
    unsigned top_left = (col - 1) * (row - 1);

    return sum_left + sum_top + top_left;
}

unsigned long Day25::get_code_at_pos(unsigned pos){

    unsigned long code = first;
    for(unsigned ii = 1; ii < pos; ii++){
        code = (code * element) % prime;
    }

    return code;
}
