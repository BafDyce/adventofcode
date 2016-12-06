#include "Day00.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

using namespace std;

Result Day00::solve_p1(){

    string result = "";

    for(unsigned ii = 0; ii < data.size(); ii++){
        result += data[ii][0];
    }

    return {true, result};
}
