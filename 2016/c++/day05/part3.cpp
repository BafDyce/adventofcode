#include "Day05.hpp"

#include <iostream>
#include <limits>

using namespace std;

Result Day05::solve_p3(){
    // see it in action here: https://youtu.be/V3In2QCFGx4
    string pw = "--------";
    string display;
    int counter = 0;

    // second new line will be overridden by first pw
    cout << "Part 2 (cinematic mode)" << endl << endl;

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
            }
        }

        // speeding up program by about 100 times
        if ( ii % 100 == 0 ){
            display = pw;
            for(unsigned cc = 0; cc < display.length(); cc++){
                if( display[cc] == '-' ){
                    display[cc] = hash[8 + cc];
                }
                cout << "\r\e[APassword: " << display << endl;
            }
        }

        if( counter == 8 ){
            return {true, pw};
        }
    }

    return {false, "error"};
}
