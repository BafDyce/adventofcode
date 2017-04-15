#include "Day18.hpp"

using namespace std;

static string get_next_row(string &row);
static char get_tile(char left, char right);
static bool is_trap(char left, char right);

Result Day18::solve_p2(){
    int safe_tiles = 0;
    for(char ch: data){
        if( ch == '.' ){
            ++safe_tiles;
        }
    }

    string row = data;
    for(unsigned ii = 1; ii < 400000; ++ii){
        row = get_next_row(row);
        for(char ch: row){
            if( ch == '.' ){
                ++safe_tiles;
            }
        }
    }

    return {true, to_string(safe_tiles)};
}

static string get_next_row(string &row){
    // result = "" + get_tile(..) didn't work (compiler just discarded the tile)
    string result = "";
    result += get_tile('.', row[1]);

    for(unsigned ii = 0; ii < row.length() - 2; ++ii){
        result += get_tile(row[ii], row[ii+2]);
    }

    result += get_tile(row[row.length() - 2], '.');
    return result;
}

static char get_tile(char left, char right){
    if( is_trap(left, right) ){
        return '^';
    }

    return '.';
}

static inline bool is_trap(char left, char right){
    if ( left == '^' ){
        return right == '.';
    } else {
        return right == '^';
    }
}
