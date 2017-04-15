#include "Day22.hpp"

#include <iostream>

using namespace std;

Result Day22::solve_p2(){
    for(unsigned ii = 0; ii < gridsize_x; ++ii){
        for(unsigned jj = 0; jj < gridsize_y; ++jj){
            Node &node = grid[ii][jj];
            if( node.used == 0 ){
                printf("___");
            } else {
                printf("%3d", node.used);
            }

            printf("/%3d ", node.size);
        }
        printf("\n");
    }

    return {true, "Look at the table and solve by hand!"};
}
