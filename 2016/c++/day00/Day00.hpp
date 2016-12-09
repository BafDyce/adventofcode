#ifndef DAY00_HPP
#define DAY00_HPP

#include <iostream>
#include <vector>

#include <../DayTemplate.hpp>
#include <vector>
#include <string>

class Day00 : public DayTemplate {
public:
    Day00(vector<string> input);
    virtual ~Day00();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

};

#endif // DAY00_HPP
