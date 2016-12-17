#ifndef DAY04_HPP
#define DAY04_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"

class Day04 : public DayTemplate
{
    public:
        Day04(vector<string>);
        virtual ~Day04();

    protected:
        Result solve_p1();
        Result solve_p2();

    private:
        vector<string> data;
};

#endif // DAY04_HPP
