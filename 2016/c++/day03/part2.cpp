#include "Day03.hpp"
#include "../AoC.hpp"

#include <vector>
#include <iostream>
#include <sstream>

struct Triangle {
    int a;
    int b;
    int c;

    bool is_valid(){
        if( (a + b) > c
                && (a + c) > b
                && (b + c) > a){
            return true;
        }

        return false;
    }
};

Result Day03::solve_p2(){
    vector<Triangle> triangles;

    for(unsigned ii = 0; ii < data.size(); ii+=3){
        stringstream r1 = stringstream(data[ii]);
        stringstream r2 = stringstream(data[ii + 1 ]);
        stringstream r3 = stringstream(data[ii + 2 ]);
        int a, b, c;
        int a2, b2, c2;
        int a3, b3, c3;
        // read values so that (a,b,c) is a triangle
        r1 >> a >> a2 >> a3;
        r2 >> b >> b2 >> b3;
        r3 >> c >> c2 >> c3;
        // using an extra nested layer to avoid copy & paste errors
        {
            Triangle tr {a, b, c};
            if( tr.is_valid() ){
                triangles.push_back(tr);
            }
        }

        {
            Triangle tr {a2, b2, c2};
            if( tr.is_valid() ){
                triangles.push_back(tr);
            }
        }

        {
            Triangle tr {a3, b3, c3};
            if( tr.is_valid() ){
                triangles.push_back(tr);
            }
        }

    }

    return {true, to_string(triangles.size())};
}
