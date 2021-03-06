#ifndef DAY20_HPP
#define DAY20_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day20 : public DayTemplate {
public:
    Day20(vector<string> input);
    virtual ~Day20();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

};

#endif // DAY20_HPP
