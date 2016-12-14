#include "Day01.hpp"
#include "../AoC.hpp"

using namespace std;

struct Santa {
    char facing;
    int x;
    int y;
};

Result Day01::solve_p1(){
    Santa santa {'N', 0, 0};

    for(unsigned ii = 0; ii < this->data.size(); ii++){
        Direction dir = this->data[ii];
        switch( santa.facing ){
            case 'N':
                if( dir.direction == 'L'){
                    santa.x -= dir.distance;
                    santa.facing = 'W';
                } else if( dir.direction == 'R' ){
                    santa.x += dir.distance;
                    santa.facing = 'E';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'E':
                if( dir.direction == 'L'){
                    santa.y += dir.distance;
                    santa.facing = 'N';
                } else if( dir.direction == 'R' ){
                    santa.y -= dir.distance;
                    santa.facing = 'S';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'S':
                if( dir.direction == 'L'){
                    santa.x += dir.distance;
                    santa.facing = 'E';
                } else if( dir.direction == 'R' ){
                    santa.x -= dir.distance;
                    santa.facing = 'W';
                } else {
                    return {false, "Input error!"};
                }
                break;
            case 'W':
                if( dir.direction == 'L'){
                    santa.y -= dir.distance;
                    santa.facing = 'S';
                } else if( dir.direction == 'R' ){
                    santa.y += dir.distance;
                    santa.facing = 'N';
                } else {
                    return {false, "Input error!"};
                }
                break;
            default:
                return {false, "Input error"};
        }
    }

    int result = abs(santa.x) + abs(santa.y);
    return {true, to_string(result) };
}
