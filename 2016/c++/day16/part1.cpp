#include "Day16.hpp"

#include <algorithm>
#include <iostream>

using namespace std;

Result Day16::solve_p1(){

    string rnd = generate_data(data, 272);
    string result = calc_checksum(rnd);

    return {true, result};
}

string Day16::generate_data(const string &source, const unsigned length){
    string result = source + "0";

    if( result.length() >= length ){
        return result.substr(0, length);
    }

    string part2 = source;
    std::reverse(part2.begin(), part2.end());
    for(unsigned ii = 0; ii < part2.length(); ++ii){
        if( part2[ii] == '1' ){
            part2[ii] = '0';
        } else if( part2[ii] == '0' ){
            part2[ii] = '1';
        }
    }

    string combined = result + part2;
    return generate_data(combined, length);
}

string Day16::calc_checksum(const string to_hash){

    if( to_hash.length() % 2 == 1 ){
        return to_hash;
    }

    string checksum = "";
    for(unsigned ii = 0; ii < to_hash.length(); ii+=2){
        if( to_hash[ii] == to_hash[ii+1] ){
            checksum += '1';
        } else {
            checksum += '0';
        }
    }

    return calc_checksum(checksum);
}
