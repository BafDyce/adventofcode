#include "Day05.hpp"

#include <iostream>
#include <limits>

#include "../util/Md5Provider.hpp"

using namespace std;

Result Day05::solve_p1(){
    Md5Provider md5;
    string pw = "";

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string hash = md5.compute(data + to_string(ii));
        if ( hash.compare(0, 5, "00000") == 0 ){
            pw += hash[5];
            if( pw.length() == 8 ){
                return {true, pw};
            }
        }
    }

    return {false, "error"};
}
