#include "Day11.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

using namespace std;

Result Day11::solve_p2(){

    string result = "";

    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        result += var;
    }

    return {true, result};
}
