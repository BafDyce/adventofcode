#include "Day20.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <limits>

// leaderboard, rank 106
Result Day20::solve_p1(){
bool *ips = new bool[std::numeric_limits<unsigned int>::max()];
    //bool ips[ std::numeric_limits<unsigned int>::max()];
    cout << std::numeric_limits<unsigned int>::max() << endl;
    cout << sizeof(bool) << endl;
    for(unsigned ii = 0; ii < std::numeric_limits<unsigned int>::max(); ++ii){
        ips[ii] = true;
    }

    // iterate over input lines
    for(string line: this->data){
        unsigned start, end;
        // ch eats the '-'
        char ch;
        stringstream stream(line);
        stream >> start >> ch >> end;

        for(unsigned ii = start; ii <= end && ii < std::numeric_limits<unsigned int>::max(); ++ii){
            ips[ii] = false;
        }
    }
    // END iterate over input lines

    for(unsigned ii = 0; ii < std::numeric_limits<unsigned>::max(); ++ii){
        if( ips[ii] ){
            delete ips;
            return {true, to_string(ii)};
        }
    }

    delete ips;
    return {false, "No valid IP found!"};
}
