#include "Day25.h"

#include <iostream>
#include <limits>

using namespace std;

unsigned long get_order(unsigned long element, unsigned long prime);

bool Day25::solve_p1(string& result) {

    unsigned pos = calc_position(row, col);
    unsigned long code = get_code_at_pos(pos);
    result = to_string(code);
    return true;
}

// helper function, to check if the given element is a generator
unsigned long get_order(unsigned long element, unsigned long prime){

    unsigned long last = element;
    unsigned long count = 1;
    do {
        last = (last * element) % prime;
        count++;
    } while( last != 1 );

    return count;
}
