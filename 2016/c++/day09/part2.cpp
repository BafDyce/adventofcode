#include "Day09.hpp"

#include <iostream>
#include <sstream>

using namespace std;

// input: the entire string
// offset: at which offset of the string we start looking at
// new_offset: where the caller should continue scanning
// returns: output length after decompressing
static unsigned long
decompress(string &input, unsigned offset, unsigned &new_offset);

Result Day09::solve_p2(){
    unsigned long result = 0;
    for(unsigned ii = 0; ii < data.length(); ii++){
        if( data[ii] == '(' ){
            result += decompress(data, ii+1, ii);
        } else {
            result++;
        }
    }

    return {true, to_string(result)};
}

// input: the entire string
// offset: at which offset of the string we start looking at
// new_offset: where the caller should continue scanning
// returns: output length after decompressing
static unsigned long
decompress(string &input, unsigned offset, unsigned &new_offset){
    int length;
    int repeat;
    char ch;

    stringstream ss(input.substr(offset));
    ss >> length >> ch >> repeat;

    while(input[offset] != ')'){
        offset++;
    }
    // also skip over ')'
    offset++;

    new_offset = offset + length -1;
    unsigned long output_length = 0;
    for(unsigned ii = offset; ii <= new_offset; ii++){
        if( input[ii] == '(' ){
            output_length += repeat * decompress(input, ii+1, ii);
        } else {
            output_length += repeat;
        }
    }

    return output_length;
}
