#include "Day00.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

using namespace std;

Result Day00::solve_p2(){

    string result = "";

    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        result += var;
    }

    return {true, result};
}
