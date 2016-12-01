#include "Day01.hpp"
#include "../AoC.hpp"

#include <vector>
#include <iostream>

using namespace std;

struct Position {
    int x;
    int y;
};

struct Santa {
    char facing;
    Position pos;
};

Result Day01::solve_p2(){
    // "visits" are each feld Santa steps over, not each position he rotates
    // Therefore, we need to turn, then walk one step at a time
    Santa santa {'N', Position{0, 0}};
    // each visited block will be saved here
    vector<Position> visits;

    for(unsigned int ii = 0; ii < this->data.size(); ii++){
        Direction dir = this->data[ii];

        // rotate
        switch( santa.facing ){
            case 'N':
                if( dir.direction == 'L'){
                    santa.facing = 'W';
                } else if( dir.direction == 'R' ){
                    santa.facing = 'E';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'E':
                if( dir.direction == 'L'){
                    santa.facing = 'N';
                } else if( dir.direction == 'R' ){
                    santa.facing = 'S';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'S':
                if( dir.direction == 'L'){
                    santa.facing = 'E';
                } else if( dir.direction == 'R' ){
                    santa.facing = 'W';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'W':
                if( dir.direction == 'L'){
                    santa.facing = 'S';
                } else if( dir.direction == 'R' ){
                    santa.facing = 'N';
                } else {
                    return {false, "Input error!"};
                }
                break;
            default:
                return {false, "Input error"};
        }

        // walk
        for(int ww = 0; ww < dir.distance; ww++){
            switch( santa.facing ){
            case 'N':
                santa.pos.y++;
                break;
            case 'E':
                santa.pos.x++;
                break;
            case 'S':
                santa.pos.y--;
                break;
            case 'W':
                santa.pos.x--;
                break;
            }

            Position pos {santa.pos.x, santa.pos.y};

            for(unsigned int vv = 0; vv < visits.size(); vv++){
                Position visited = visits[vv];
                if( (visited.x == pos.x && visited.y == pos.y) ){
                    int result = abs(pos.x) + abs(pos.y);
                    return {true, to_string(result)};
                }
            }

            visits.push_back(pos);
        }

    }

    return {false, "Computation error!" };
}
