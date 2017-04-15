#include "Day22.hpp"

using namespace std;

Result Day22::solve_p1(){
    int counter = 0;
    for(unsigned ii = 0; ii < gridsize_x * gridsize_y; ++ii){
        Node &first = grid[ii/gridsize_y][ii % gridsize_y];

        for(unsigned jj = ii + 1; jj < gridsize_x * gridsize_y; ++jj){
            Node &second = grid[ jj/gridsize_y ][jj % gridsize_y];
            if( first.used != 0 && first.used <= second.avail() ){
                ++counter;
            }

            if( second.used != 0 && second.used <= first.avail() ){
                ++counter;
            }
        }
    }

    return {true, to_string(counter)};
}
