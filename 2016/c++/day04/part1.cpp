#include "Day04.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

struct Pair {
    char ch;
    int n;
};

static bool sortPair(Pair a, Pair b){
    if( a.n == b.n ){
        return a.ch < b.ch;
    }

    return a.n > b.n;
}

Result Day04::solve_p1(){

    int result = 0;

    for(unsigned ii = 0; ii < this->data.size(); ii++){
        string content;
        stringstream line(this->data[ii]);
        line >> content;

        Pair letters[26] = {0, 0};
        for( unsigned jj = 0; jj < 26; jj++){
            letters[jj].ch = 'a' + jj;
        }

        int id = 0;
        string checksum = "";
        for(unsigned jj = 0; jj < content.length(); jj++){
            char letter = content[jj];
            if( letter == '-' ){
                continue;
            } else if ( letter >= 'a' && letter <= 'z' ){
                letters[ letter - 'a' ].n++;
            } else if ( letter >= '0' && letter <= '9' && id == 0 ){
                stringstream numberString(content.substr(jj));
                numberString >> id;
            } else if ( letter == '[') {
                checksum = content.substr(jj+1);
                checksum.pop_back();
                break;
            }
        }

        sort(letters, letters + 26, sortPair);
        string check = "";
        for( unsigned jj = 0; jj < 5; jj++){
            check += letters[jj].ch;
        }
        cout << "check: " << check << " checksum: " << checksum << endl;
        if( checksum == check ){
            result += id;
        }
    }

    return {true, to_string(result)};
}
