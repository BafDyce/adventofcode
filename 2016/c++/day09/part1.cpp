#include "Day09.hpp"

#include <iostream>
#include <sstream>

using namespace std;

Result Day09::solve_p1(){
    string result = "";
    string to_repeat = "";
    int repeat = 0;

    for(unsigned ii = 0; ii < data.length(); ii++){
        char ch = data[ii];

        if( ch == '(' ) {
            int length;
            char x;
            stringstream str( data.substr(ii + 1) );
            str >> length >> x >> repeat;
            // search for )
            while(data[ii] != ')'){
                ii++;
            }
            // also skip over ')'
            ++ii;

            to_repeat = "";
            for(int jj = 0; jj < length; jj++, ii++){
                to_repeat += data[ii];
            }

            for(int jj = 0; jj < repeat; jj++){
                result += to_repeat;
            }

            ii--;
        } else {
            result += ch;
        }
    }

    return {true, to_string(result.length())};
}
