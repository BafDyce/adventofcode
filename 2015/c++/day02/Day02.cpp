#include "Day02.h"

#include <iostream>
#include <vector>

using namespace std;

Day02::Day02() {
}

Day02::Day02(vector<string> input){
    data = input;
}

Day02::~Day02(){
}

bool Day02::solve(int part, string& result){
    switch(part){
    case 1:
        return solve_p1(result);
    case 2:
        return solve_p2(result);
    }

    return false;
}
