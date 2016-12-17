#include "Day14.hpp"

#include <vector>
#include <iostream>

Day14::Day14(vector<string> input) : data(input[0]) {
}

Day14::~Day14() {
}

char Day14::get_first_tripple(string &hash){
    vector<char> last;
    last.push_back('\0');
    for(unsigned ii = 0; ii < 32; ++ii){
        char ch = hash[ii];
        if( ch != last[ last.size() - 1 ] ){
            last.erase(last.begin(), last.begin() + last.size());
            last.push_back(ch);
        } else {
            last.push_back(ch);
            if( last.size() == 3 ){
                return ch;
            }
        }
    }

    return '\0';
}

vector<char> Day14::get_quintuples(string &hash){
    vector<char> quintuples;
    vector<char> last;
    last.push_back('\0');
    for(unsigned ii = 0; ii < 32; ++ii){
        char ch = hash[ii];
        if( ch != last[ last.size() - 1 ] ){
            last.erase(last.begin(), last.begin() + last.size());
            last.push_back(ch);
        } else {
            last.push_back(ch);
            if( last.size() == 5 ){
                quintuples.push_back(ch);
                last.erase(last.begin(), last.begin() + last.size());
            }
        }
    }
    last.erase(last.begin(), last.begin() + 0);

    return quintuples;
}

bool Day14::is_key(vector<Key> hits[16], string &hash, int counter, vector<int> &result){
    vector<char> quintuple = get_quintuples(hash);
    bool retval = false;

    for(char ch: quintuple){
        unsigned idx = 0;
        if( ch >= '0' && ch <= '9'){
            idx = ch - '0';
        } else if ( ch >= 'a' && ch <= 'f'){
            idx = ch - 'a' + 10;
        } else {
            cerr << "ERROR! Wrong char: " << ch << endl;
        }

        for(Key key: hits[idx]){
            if(  key.counter >= (counter - 1000)){
                result.push_back(key.counter);
            }

            if( result.size() > 0 ){
                retval = true;
            }
        }
    }

    char ch = get_first_tripple(hash);
    if( ch != '\0' ){
        unsigned idx = 0;
        if( ch >= '0' && ch <= '9'){
            idx = ch - '0';
        } else if ( ch >= 'a' && ch <= 'f'){
            idx = ch - 'a' + 10;
        } else {
            cerr << "ERROR! Wrong char: " << ch << endl;
        }

        hits[idx].push_back(Key{hash, ch, counter});
    }

    return retval;
}
