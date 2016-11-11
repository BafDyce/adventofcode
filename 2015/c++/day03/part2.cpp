#include "Day03.h"

#include <iostream>
#include <unordered_map>

using namespace std;

bool Day03::solve_p2(string& result){

    unordered_map<Position, bool> grid;

    Position santa {0, 0};
    Position robot {0, 0};

    grid.insert({santa, true});

    for( unsigned int ii = 0; ii < data.length(); ii += 2){
        santa.walk(data[ii]);
        robot.walk(data[ii + 1]);

        grid.insert({santa, true});
        grid.insert({robot, true});
    }

    result = to_string( grid.size() );
    return true;
}
