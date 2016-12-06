#include "Day05.hpp"

#include <iostream>
#include <limits>

using namespace std;

Result Day05::solve_p1(){
    string pw = "";

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string hash = compute_md5(data + to_string(ii));
        if ( hash.compare(0, 5, "00000") == 0 ){
            pw += hash[5];
            if( pw.length() == 8 ){
                return {true, pw};
            }
        }
    }

    return {false, "error"};
}
