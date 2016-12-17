#ifndef DAY10_HPP
#define DAY20_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day10 : public DayTemplate {
public:
    Day10(vector<string> input);
    virtual ~Day10();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

};

#endif // DAY00_HPP
