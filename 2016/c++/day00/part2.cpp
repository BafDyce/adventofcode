#include "Day00.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <limits>
#include <queue>

#include "../util/Md5Provider.hpp"

using namespace std;

Result Day00::solve_p2(){
    Md5Provider md5;
    string result = "";

    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        result += md5.compute(var);
    }

    return {true, result};
}
