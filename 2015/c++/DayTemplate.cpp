#include "DayTemplate.h"

#include <iostream>
#include <vector>

using namespace std;

DayTemplate::DayTemplate() {
}

DayTemplate::DayTemplate(const vector<string> input) {
    data = input;
}

DayTemplate::~DayTemplate() {
}

bool DayTemplate::solve(const int part, string& result) {
    switch(part){
    case 1:
        return solve_p1(result);
    case 2:
        return solve_p2(result);
    }

    return false;
}

bool DayTemplate::solve_p1(string& result){
    return false;
}

bool DayTemplate::solve_p2(string& result){
    return false;
}
