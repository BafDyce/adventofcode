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

        Pair letters[26] = {0, 0};
        for( unsigned jj = 0; jj < 26; jj++){
            letters[jj].ch = 'a' + jj;
        }

        int id = 0;
        string checksum = "";
        string name = "";
        for(unsigned jj = 0; jj < content.length(); jj++){
            char letter = content[jj];
            if( letter == '-' ){
                name += letter;
                continue;
            } else if ( letter >= 'a' && letter <= 'z' ){
                name += letter;
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

        if( checksum == check ){
            string decrypted = decrypt(name, id);
            if( decrypted == "northpole object storage " ){
                result = to_string(id);
            }
        }
    }

    return {true, result};
}
/*
static bool sortPair(Pair a, Pair b){
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
}*/
