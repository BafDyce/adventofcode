#include "Day15.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <limits>

using namespace std;

struct Disc {
    int positions;
    int pos;

    // rotates the disc excatly one slot forward
    int rotate(){
        pos = (pos + 1) % positions;
        return pos;
    }

    // returns the position the disc would have in n seconds
    int pseudo_rotate(unsigned n){
        return ( (pos + n) % positions );
    }
};

Result Day15::solve_p2(){
    vector<Disc> discs;

    // I SHOULD LEARN HOW TO USE REGEX!!
    for(string line: this->data){
        // skip beginning of line
        stringstream stream(line.substr(12));
        // read disc positions
        int positions = 0;
        stream >> positions;
        // skip "positions"
        string tmp;
        stream >> tmp;
        // read until 'n' (character before starting position)
        char ch = ' ';
        do {
            stream >> ch;
        } while(ch != 'n');
        // read start positions
        int startpos = 0;
        stream >> startpos;

        discs.push_back({positions, startpos});
    }
    // additional disc
    discs.push_back({11, 0});

    for(unsigned int ii = 1; ii < numeric_limits<unsigned int>::max(); ++ii){
        // rotate each disc one slot
        for(Disc &disc: discs){
            disc.rotate();
        }

        int check = 0;
        for(unsigned kk = 0; kk < discs.size(); ++kk){
            if( discs[kk].pseudo_rotate(kk + 1) == 0 ){
                ++check;
            } else {
                // break for efficiency
                break;
            }
        }

        if( check == discs.size() ){
            return {true, to_string(ii)};
        }
    }

    return {false, "No suitable time slot found!"};
}


