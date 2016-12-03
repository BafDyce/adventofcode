#ifndef DAY01_HPP
#define DAY01_HPP

#include <iostream>
#include <sstream>
#include <vector>

#include <../DayTemplate.hpp>

struct Direction {
    char direction;
    int distance;
};

class Day01 : public DayTemplate {
    public:
        Day01(vector<string>);
        virtual ~Day01();

    protected:
        Result solve_p1();
        Result solve_p2();

    private:
        vector<Direction> data;
};

#endif // DAY01_HPP
