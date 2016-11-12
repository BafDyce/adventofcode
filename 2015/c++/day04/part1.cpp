#include "Day04.h"

#include <iostream>
#include <limits>

using namespace std;

bool Day04::solve_p1(string& result) {

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string coin = compute_md5(data + to_string(ii));
        if ( coin.compare(0, 5, "00000") == 0 ){
            result = to_string(ii);
            return true;
        }
    }

    return false;
}
