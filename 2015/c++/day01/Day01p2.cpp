#include "Day01p2.h"

#include <iostream>
#include <vector>

using namespace std;

Day01p2::Day01p2(vector<string> input) {
    // we just want the first (and only) line
    data = input[0];
}

Day01p2::~Day01p2() {
    //dtor
}

bool Day01p2::solve(string& result){
    int floor = 0;
    unsigned int ii;
    for(ii = 0; ii < data.length() && floor != -1; ii++){
        switch(data[ii]){
        case '(':
            floor++;
            break;
        case ')':
            floor--;
            break;
        }
    }

    if( floor == -1 ){
        result = to_string(ii);
        return true;
    }

    return false;
}
