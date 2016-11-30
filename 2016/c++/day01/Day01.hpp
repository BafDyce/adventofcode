#ifndef DAY01_HPP
#define DAY01_HPP

#include <iostream>
#include <vector>

#include <../DayTemplate.hpp>


class Day01 : public DayTemplate {
    public:
        Day01(vector<string>);
        virtual ~Day01();

    protected:
        Result solve_p1();
        Result solve_p2();

    private:
};

#endif // DAY01_HPP
