#include "Day20.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <limits>


using namespace std;

// leaderboard, rank 29!!
Result Day20::solve_p2(){
    bool *ips = new bool[std::numeric_limits<unsigned int>::max()];
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

    unsigned counter = 0;
    for(unsigned ii = 0; ii < std::numeric_limits<unsigned>::max(); ++ii){
        if( ips[ii] ){
            counter++;
        }
    }

    delete ips;
    return {true, to_string(counter)};
}
