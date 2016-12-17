#include "Day05.hpp"

#include <vector>
#include <iostream>

Day05::Day05(vector<string> input) : data(input[0])  {
}

Day05::~Day05() {
}

Result Day05::solve(const int part) {
    if( part == 3 ) {
        return this->solve_p3();
    }

    return DayTemplate::solve(part);
}
