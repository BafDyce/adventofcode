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

        Pair letters[26] = {0};
        for( unsigned ii = 0; ii < 26; ii++){
            letters[ii].ch = 'a' + ii;
        }

        int id = 0;
        string checksum = "";
        for(unsigned ii = 0; ii < content.length(); ii++){
            char letter = content[ii];
            if( letter == '-' ){
                continue;
            } else if ( letter >= 'a' && letter <= 'z' ){
                letters[ letter - 'a' ].n++;
            } else if ( letter >= '0' && letter <= '9' && id == 0 ){
                stringstream numberString(content.substr(ii));
                numberString >> id;
            } else if ( letter == '[') {
                checksum = content.substr(ii+1);
                checksum.pop_back();
                break;
            }
        }

        sort(letters, letters + 26, sortPair);
        string check = "";
        for( unsigned ii = 0; ii < 5; ii++){
            check += letters[ii].ch;
        }
        cout << "check: " << check << " checksum: " << checksum << endl;
        if( checksum == check ){
            result += id;
        }
    }

    return {true, to_string(result)};
}
