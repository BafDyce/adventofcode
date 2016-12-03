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

Result Day03::solve_p1(){
    vector<Triangle> triangles;

    for(unsigned ii = 0; ii < this->data.size(); ii++){
        stringstream triangle = stringstream(data[ii]);
        int a, b, c;
        triangle >> a >> b >> c;
        Triangle tr {a, b, c};
        if( tr.is_valid() ){
            triangles.push_back(tr);
        }
    }

    return {true, to_string(triangles.size())};
}
