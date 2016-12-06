#include "Day06.hpp"

using namespace std;

Result Day06::solve_p1(){
    int amount[8][26] = {0};

    for(unsigned ii = 0; ii < data.size(); ii++){
        string code = data[ii];

        for(unsigned jj = 0; jj < code.length(); jj++){
            char x = code[jj];
            amount[jj][ x - 'a' ]++;
        }
    }

    string result = "";
    for(unsigned ii = 0; ii < 8; ii++){
        int max = -1;
        char maxChar = ' ';
        for(unsigned jj = 0; jj < 26; jj++){
            int x = amount[ii][jj];
            if( x > max ){
                max = x;
                maxChar = (char) (jj + 'a');
            }
        }
        result += maxChar;
    }

    return {true, result};
}
