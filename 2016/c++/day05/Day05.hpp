#ifndef DAY05_HPP
#define DAY05_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"

class Day05 : public DayTemplate
{
    public:
        Day05(vector<string>);
        virtual ~Day05();
        Result solve(const int part);

    protected:
        Result solve_p1();
        Result solve_p2();
        Result solve_p3();

    private:
        string data;
};

#endif // DAY05_HPP
