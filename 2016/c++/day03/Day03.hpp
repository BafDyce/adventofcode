#ifndef DAY03_HPP
#define DAY03_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"


class Day03 : public DayTemplate
{
    public:
        Day03(vector<string>);
        virtual ~Day03();

    protected:
        Result solve_p1();
        Result solve_p2();

    private:
        vector<string> data;
};

#endif // DAY03_HPP
