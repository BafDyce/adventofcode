#ifndef DAY23_HPP
#define DAY23_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"
#include "../util/Assembunny.hpp"

class Day23 : public DayTemplate {
public:
    Day23(vector<string> input);
    virtual ~Day23();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<Instruction> instructions;
};

#endif // DAY23_HPP
