#include "Day04.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>

struct Pair {
    char ch;
    int n;
};

bool sortPair(Pair a, Pair b){
    if( a.n == b.n ){
        return a.ch < b.ch;
    }

    return a.n > b.n;
}

static string decrypt(string name, int salt){
    string result = "";
    for(unsigned ii = 0; ii < name.length(); ii++){
        if ( name[ii] == '-' ){
            result += ' ';
            continue;
        }

        int letter = name[ii] + salt % 26;
        if( letter > 'z' ){
            int tmp = letter - 'z' + 'a' - 1;
            result += (char) tmp;
        } else if( letter >= 'a' && letter <= 'z' ){
            result += letter;
        }
    }

    return result;
}

Result Day04::solve_p2(){

    string result = "error";

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
        string name = "";
        for(unsigned ii = 0; ii < content.length(); ii++){
            char letter = content[ii];
            if( letter == '-' ){
                name += letter;
                continue;
            } else if ( letter >= 'a' && letter <= 'z' ){
                name += letter;
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

        if( checksum == check ){
            string decrypted = decrypt(name, id);
            if( decrypted == "northpole object storage " ){
                result = to_string(id);
            }
        }
    }

    return {true, result};
}
