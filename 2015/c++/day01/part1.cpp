#include "Day01.h"

#include <iostream>
#include <vector>

using namespace std;

unsigned int calc_paper_for_present(const string present);

bool Day01::solve_p1(string& result){
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
