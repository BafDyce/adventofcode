#include "Day20.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <limits>
#include <cstring>

using namespace std;

// leaderboard, rank 29!!
/*
Command being timed: "./bin/Release/aoc2016 --day 20 --inputset puzzle1 --part 2"
User time (seconds): 6.33
System time (seconds): 0.83
Percent of CPU this job got: 99%
Elapsed (wall clock) time (h:mm:ss or m:ss): 0:07.17
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 4200408
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 1048945
Voluntary context switches: 1
Involuntary context switches: 11
Swaps: 0
File system inputs: 0
File system outputs: 0
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0
*/
Result Day20::solve_p2(){
    unsigned length = std::numeric_limits<unsigned int>::max();
    bool *ips = new bool[length];
    // original (leaderboard) solution had a loop instead of memset()
    memset(ips, true, sizeof(bool) * length);

    // iterate over input lines
    for(string line: this->data){
        unsigned start, end;
        // ch eats the '-'
        char ch;
        stringstream stream(line);
        stream >> start >> ch >> end;

        for(unsigned ii = start; ii <= end && ii < length; ++ii){
            ips[ii] = false;
        }
    }
    // END iterate over input lines

    unsigned counter = 0;
    for(unsigned ii = 0; ii < length; ++ii){
        if( ips[ii] ){
            counter++;
        }
    }

    delete ips;
    return {true, to_string(counter)};
}
