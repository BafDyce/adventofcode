#ifndef DAY15_HPP
#define DAY15_HPP

#include <../DayTemplate.hpp>

#include <vector>
#include <iostream>

class Day15 : public DayTemplate {
public:
    Day15(vector<string>);
    virtual ~Day15();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

};

#endif // DAY15_HPP
