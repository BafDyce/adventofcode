#include "Day05.hpp"

#include <iostream>
#include <limits>

using namespace std;

Result Day05::solve_p2(){
    string pw = "--------";
    int counter = 0;

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string hash = compute_md5(data + to_string(ii));
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
