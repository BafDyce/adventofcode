#include "Day01p1.h"

#include <iostream>
#include <vector>

using namespace std;

Day01p1::Day01p1(vector<string> input) {
    // we just want the first (and only) line
    data = input[0];
}

Day01p1::~Day01p1() {
    //dtor
}

bool Day01p1::solve(string& result){
    int floor = 0;
    for(unsigned int ii = 0; ii < data.length(); ii++){
        switch(data[ii]){
        case '(':
            floor++;
            break;
        case ')':
            floor--;
            break;
        }
    }

    result = to_string(floor);
    return true;
}
