#ifndef DAY02_HPP
#define DAY02_HPP

#include <../DayTemplate.hpp>

#include <vector>
#include <iostream>

class Day02 : public DayTemplate {
    public:
        Day02(vector<string>);
        virtual ~Day02();

    protected:
        Result solve_p1();
        Result solve_p2();

    private:
        vector<string> data;
};

#endif // DAY02_HPP
