#include "Day06.hpp"

#include <limits>

using namespace std;

Result Day06::solve_p2(){
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
        int min = numeric_limits<int>::max();
        char minChar = '-';
        for(unsigned jj = 0; jj < 26; jj++){
            int x = amount[ii][jj];
            if( x < min){
                min = x;
                minChar = (char) (jj + 'a');
            }
        }
        result += minChar;
    }

    return {true, result};
}
