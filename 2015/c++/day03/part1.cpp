#include "Day03.h"

#include <iostream>
#include <unordered_map>

using namespace std;

bool Day03::solve_p1(string& result){

    unordered_map<Position, bool> grid;
    Position santa {0, 0};
    grid.insert({santa, true});

    for( unsigned int ii = 0; ii < data.length(); ii++){
        santa.walk(data[ii]);
        grid.insert({santa, true});
    }

    result = to_string( grid.size() );
    return true;
}
