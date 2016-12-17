#ifndef DAY09_HPP
#define DAY09_HPP

#include <vector>
#include <string>
#include <iostream>

#include "../DayTemplate.hpp"

class Day09 : public DayTemplate {
public:
    Day09(vector<string> input);
    virtual ~Day09();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;

};

#endif // DAY09_HPP
