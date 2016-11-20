#include "Day06.h"

#include <vector>
#include <iostream>

Day06::Day06()
{
    //ctor
}

Day06::Day06(vector<string> input){
    data = input;
    for(unsigned ii = 0; ii < input.size(); ii++){
        Instruction instr(input[ii]);
        instructions.push_back(instr);
    }
}

Day06::~Day06()
{
    //dtor
}

int Day06::count_lights(){
    int ctr = 0;
    for(unsigned ii = 0; ii < gridsize; ii++){
        for(unsigned jj = 0; jj < gridsize; jj++){
            ctr += grid[ii][jj];
        }
    }

    return ctr;
}

void Day06::turn_on(Day6Position from, Day6Position to){
    for(unsigned xx = from.x; xx <= to.x; xx++){
        for(unsigned yy = from.y; yy <= to.y; yy++){
            grid[xx][yy] = 1;
        }
    }
}

void Day06::turn_off(Day6Position from, Day6Position to){
    for(unsigned xx = from.x; xx <= to.x; xx++){
        for(unsigned yy = from.y; yy <= to.y; yy++){
            grid[xx][yy] = 0;
        }
    }
}

void Day06::toggle(Day6Position from, Day6Position to){
    for(unsigned xx = from.x; xx <= to.x; xx++){
        for(unsigned yy = from.y; yy <= to.y; yy++){
            grid[xx][yy] = (grid[xx][yy] + 1) % 2;
        }
    }
}

void Day06::brighten(Day6Position from, Day6Position to, int value){
    for(unsigned xx = from.x; xx <= to.x; xx++){
        for(unsigned yy = from.y; yy <= to.y; yy++){
            grid[xx][yy] += value;
        }
    }
}

void Day06::dim(Day6Position from, Day6Position to){
    for(unsigned xx = from.x; xx <= to.x; xx++){
        for(unsigned yy = from.y; yy <= to.y; yy++){
            if( grid[xx][yy] != 0) {
                grid[xx][yy] -= 1;
            }
        }
    }
}
