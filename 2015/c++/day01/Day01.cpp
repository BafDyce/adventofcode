#include "Day01.h"

#include <iostream>
#include <vector>

using namespace std;

Day01::Day01() {
}

Day01::Day01(vector<string> input){
    data = input[0];
}

Day01::~Day01(){
}

bool Day01::solve(int part, string& result){
    switch(part){
    case 1:
        return solve_p1(result);
    case 2:
        return solve_p2(result);
    }

    return false;
}
