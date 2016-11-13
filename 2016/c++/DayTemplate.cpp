#include "DayTemplate.hpp"

#include <iostream>
#include <vector>

#include "AoC.hpp"

using namespace std;

DayTemplate::DayTemplate() {
}

DayTemplate::DayTemplate(const vector<string> input) {
    data = input;
}

DayTemplate::~DayTemplate() {
}

Result DayTemplate::solve(const int part) {
    switch(part){
    case 1:
        return solve_p1();
    case 2:
        return solve_p2();
    }

    return {false,"Specified part (" + to_string(part) + ") is not available!"};
}

Result DayTemplate::solve_p1(){
    return {false, "Part 1 is not implemented!"};
}

Result DayTemplate::solve_p2(){
    return {false, "Part 2 is not implemented!"};
}
