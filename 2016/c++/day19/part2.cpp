#include "Day19.hpp"

#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

struct Elf {
    int id;
};

/*
Command being timed: "./bin/Release/aoc2016 --day 19 --inputset puzzle1 --part 2"
User time (seconds): 1724.60
System time (seconds): 0.06
Percent of CPU this job got: 99%
Elapsed (wall clock) time (h:mm:ss or m:ss): 28:47.52
Average shared text size (kbytes): 0
Average unshared data size (kbytes): 0
Average stack size (kbytes): 0
Average total size (kbytes): 0
Maximum resident set size (kbytes): 38744
Average resident set size (kbytes): 0
Major (requiring I/O) page faults: 0
Minor (reclaiming a frame) page faults: 14380
Voluntary context switches: 1
Involuntary context switches: 27963
Swaps: 0
File system inputs: 0
File system outputs: 0
Socket messages sent: 0
Socket messages received: 0
Signals delivered: 0
Page size (bytes): 4096
Exit status: 0
*/
Result Day19::solve_p2(){
    stringstream stream(data);
    unsigned amount = 0;
    stream >> amount;

    vector<Elf> elves;

    for(unsigned ii = 0; ii < amount; ++ii){
        elves.push_back({ii + 1});
    }

    unsigned stealer = 0;
    unsigned elves_in_game = amount;
    while( elves_in_game > 1 ){
        if( elves_in_game % 100000 == 0){
            cout << elves_in_game << " elves left." << endl;
        }
        // get victim at opposite end of circle
        int idx = (stealer + elves_in_game / 2) % elves_in_game;

        // elves.erase() is painfully slow!!
        elves.erase( elves.begin() + idx );
        --elves_in_game;

        // go to next stealer
        if ( idx > stealer ){
            stealer = (stealer + 1) % elves_in_game;
        } else {
            stealer = stealer % elves_in_game;
        }
    }

    return {true, to_string(elves[0].id)};
}
