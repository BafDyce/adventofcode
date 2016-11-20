#include "Day05.h"

bool is_nice_string(string& str);
bool has_at_least_three_vowels(string& str);
bool has_double_letter(string& str);
bool contains_naughty_substrings(string& str);

bool Day05::solve_p1(string& result){

    unsigned ctr = 0;
    for(unsigned ii = 0; ii < data.size(); ii++){
        if( is_nice_string(data[ii]) ){
            ctr++;
        }
    }

    result = to_string(ctr);
    return true;
}

bool is_nice_string(string& str){
    return has_at_least_three_vowels(str)
        && has_double_letter(str)
        && ! contains_naughty_substrings(str);
}

bool has_at_least_three_vowels(string& str){
    unsigned vowels = 0;
    for(unsigned ii = 0; ii < str.length(); ii++){
        switch(str[ii]){
        case 'a':
        case 'e':
        case 'i':
        case 'o':
        case 'u':
            vowels++;
        }
    }

    return vowels >= 3;
}

bool has_double_letter(string& str){
    char last = '\0';
    for(unsigned ii = 0; ii < str.length(); ii++){
        char cur = str[ii];
        if( cur == last ){
            return true;
        } else {
            last = cur;
        }
    }

    return false;
}

bool contains_naughty_substrings(string& str){
    vector<string> naughty_strings = {"ab", "cd", "pq", "xy"};

    unsigned check = 0;
    for(unsigned ii = 0; ii < naughty_strings.size(); ii++){
        if( str.find(naughty_strings[ii]) == string::npos ){
            check++;
        }
    }

    return check != naughty_strings.size();
}
