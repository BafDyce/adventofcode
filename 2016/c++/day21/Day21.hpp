#ifndef DAY21_HPP
#define DAY21_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day21 : public DayTemplate {
public:
    Day21(vector<string> input);
    virtual ~Day21();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;

    string scramble_pw(string &pw);

};

#endif // DAY21_HPP
