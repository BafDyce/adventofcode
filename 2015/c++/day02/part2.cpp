#include "Day02.h"

#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm> // min_element()

using namespace std;

unsigned int calc_ribbon_for_present(const string box);

bool Day02::solve_p2(string& result){
    unsigned int ribbon = 0;
    unsigned int length = data.size();
    for( unsigned int ii = 0; ii < length; ii++){
        ribbon += calc_ribbon_for_present(data[ii]);
    }

    result = to_string(ribbon);
    return true;
}

unsigned int calc_ribbon_for_present(const string present){
    // parsing input
    stringstream input = stringstream(present);
    unsigned int length;
    unsigned int width;
    unsigned int height;
    char x; // ununsed variable for eating the 'x's
    input >> length >> x >> width >> x >> height;

    unsigned int ribbon = length * width * height;

    // put dimensions into a vector, so we can extract the lower two values
    vector<unsigned int> sides{length, width, height};
    // sort values, so that largest value is last
    sort(sides.begin(), sides.end());

    for(unsigned int ii = 0; ii < sides.size() - 1; ii++){
        ribbon += (2 * sides[ii]);
    }

    return ribbon;
}
