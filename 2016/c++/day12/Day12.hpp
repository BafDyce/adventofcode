#ifndef DAY12_HPP
#define DAY12_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"
#include "Instruction.hpp"

class Day12 : public DayTemplate {
public:
    Day12(vector<string> input);
    virtual ~Day12();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<Instruction> instructions;

};

#endif // DAY12_HPP
