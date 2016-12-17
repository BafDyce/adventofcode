#include "Day16.hpp"

#include <iostream>

using namespace std;

// Implementation of these functions can be found in ./part1.cpp
// string generate_data(string &source, const int length);
// string calc_checksum(string data);

Result Day16::solve_p2(){

    string rnd = generate_data(data, 35651584);
    string result = calc_checksum(rnd);

    return {true, result};
}
