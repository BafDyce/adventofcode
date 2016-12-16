#ifndef DAY16_HPP
#define DAY16_HPP

#include <iostream>
#include <vector>

#include <../DayTemplate.hpp>
#include <vector>
#include <string>

class Day16 : public DayTemplate {
public:
    Day16(vector<string> input);
    virtual ~Day16();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;

    // implementation in part1.cpp
    string generate_data(const string &source, const unsigned length);
    string calc_checksum(const string data);
};

#endif // DAY16_HPP
