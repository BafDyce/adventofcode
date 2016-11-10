#include "Day01p2.h"

#include <iostream>
using namespace std;

Day01p2::Day01p2(string input) {
    data = input;
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
