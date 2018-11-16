#ifndef DAY25_HPP
#define DAY25_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"
#include "../util/Assembunny.hpp"

class Day25 : public DayTemplate {
public:
    Day25(vector<string> input);
    virtual ~Day25();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<Instruction> instructions;
};

#endif // DAY25_HPP
