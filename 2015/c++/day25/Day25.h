#ifndef DAY25_H
#define DAY25_H

#include <iostream>
#include <vector>

#include "../DayTemplate.h"

using namespace std;


class Day25 : public DayTemplate {
    public:
        Day25(vector<string> input);
        virtual ~Day25();

    protected:
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        string data;
        unsigned row;
        unsigned col;
        const unsigned long prime = 33554393;
        const unsigned long element = 252533;
        const unsigned long first = 20151125;

        Day25();
        unsigned calc_consecutive_sum_from_1_to(const unsigned n);
        unsigned calc_position(const unsigned col, const unsigned row);
        unsigned long get_code_at_pos(unsigned pos);

};

#endif // DAY25_H
