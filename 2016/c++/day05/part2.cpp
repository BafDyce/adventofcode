#include "Day05.hpp"

#include <iostream>
#include <limits>

#include "../util/Md5Provider.hpp"

using namespace std;

Result Day05::solve_p2(){
    Md5Provider md5;
    string pw = "--------";
    int counter = 0;

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string hash = md5.compute(data + to_string(ii));
        if ( hash.compare(0, 5, "00000") == 0 ){
            int index = hash[5] - '0';
            if( index > 7 ){
                continue;
            }

            if( pw[index] == '-' ){
                pw[index] = hash[6];
                counter++;
                if( counter == 8 ){
                    return {true, pw};
                }
            }
        }
    }

    return {false, "error"};
}
