#include "Day01.h"

#include <iostream>
#include <vector>

using namespace std;

unsigned int calc_paper_for_present(const string present);

bool Day01::solve_p2(string& result){
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
