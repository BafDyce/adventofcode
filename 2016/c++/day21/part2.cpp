#include "Day21.hpp"

#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

Result Day21::solve_p2(){
    string hashed = "fbgdceah";

    string password("abcdefgh");
    do {
        if( scramble_pw(password) == hashed ){
            return {true, password};
        }
    } while ( std::next_permutation(password.begin(), password.end()) );

    return {false, "No password found!"};
}
