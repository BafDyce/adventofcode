#ifndef DAY11_HPP
#define DAY11_HPP

#include <iostream>
#include <vector>

#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day11 : public DayTemplate {
public:
    Day11(vector<string> input);
    virtual ~Day11();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

};

#endif // DAY11_HPP
