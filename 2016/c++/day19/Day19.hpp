#ifndef DAY19_HPP
#define DAY19_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day19 : public DayTemplate {
public:
    Day19(vector<string> input);
    virtual ~Day19();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;

};

#endif // DAY19_HPP
