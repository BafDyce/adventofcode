#include "Day05.h"

bool is_nice_string2(string& str);
bool contains_any_pair_twice(string& str);
bool contains_separated_pair(string& str);

bool Day05::solve_p2(string& result){
    unsigned ctr = 0;
    for(unsigned ii = 0; ii < data.size(); ii++){
        if( is_nice_string2(data[ii]) ){
            ctr++;
        }
    }

    result = to_string(ctr);
    return true;
}

bool is_nice_string2(string& str){
    return contains_any_pair_twice(str) && contains_separated_pair(str);
}

bool contains_any_pair_twice(string& str){
    for(unsigned ii = 0; ii < str.length() - 3; ii++){
        string pair = str.substr(ii, 2);
        if( str.find(pair, ii + 2) != string::npos ){
            return true;
        }
    }

    return false;
}

bool contains_separated_pair(string& str){
    for(unsigned ii = 0; ii < str.length() - 2; ii++){
        if( str[ii] == str[ii + 2] ){
            return true;
        }
    }

    return false;
}
