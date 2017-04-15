#ifndef DAY18_HPP
#define DAY18_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day18 : public DayTemplate {
public:
    Day18(vector<string> input);
    virtual ~Day18();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;

};

#endif // DAY18_HPP
