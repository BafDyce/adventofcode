#ifndef DAY07_HPP
#define DAY07_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"


class Day07 : public DayTemplate {
public:
    Day07(vector<string>);
    virtual ~Day07();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;
};

#endif // DAY07_HPP
