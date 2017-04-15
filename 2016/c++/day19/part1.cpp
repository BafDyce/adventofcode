#include "Day19.hpp"

#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

struct Elf {
    int id;
    int presents;
};

/*
Command being timed: "./bin/Release/aoc2016 --day 19 --inputset puzzle1 --part 1"
User time (seconds): 0.37
System time (seconds): 0.01
Percent of CPU this job got: 82%
Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.46
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 38792
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 7
Minor (reclaiming a frame) page faults: 14376
Voluntary context switches: 9
Involuntary context switches: 2
Swaps: 0
File system inputs: 736
File system outputs: 0
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0
*/
Result Day19::solve_p1(){
    stringstream stream(data);
    unsigned amount = 0;
    stream >> amount;

    vector<Elf> elves;

    for(unsigned ii = 0; ii < amount; ++ii){
        elves.push_back({ii + 1, 1});
    }

    unsigned idx = 0;
    unsigned removed = 0;
    while( removed < amount - 1 ){
        do {
            idx = (idx + 1) % amount;
        } while( elves[idx].presents == -1 );

        elves[idx].presents = -1;
        ++removed;

        // go to next stealer
        do {
            idx = (idx + 1) % amount;
        } while( elves[idx].presents == -1 );
    }

    int counter = 0;
    for(unsigned ii = 0; ii < amount; ++ii){
        if( elves[ii].presents != -1 ){
            return {true, to_string(elves[ii].id)};
        }
    }

    return {false, "An error occured"};
}
