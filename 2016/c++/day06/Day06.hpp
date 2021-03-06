#ifndef DAY06_HPP
#define DAY06_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"


class Day06 : public DayTemplate {
public:
    Day06(vector<string> input);
    virtual ~Day06();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;
};

#endif // DAY06_HPP
