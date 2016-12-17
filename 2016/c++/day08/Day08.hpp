#ifndef DAY08_HPP
#define DAY08_HPP

#include <vector>
#include <string>

#include "../DayTemplate.hpp"

class Day08 : public DayTemplate {
public:
    Day08(vector<string> input);
    virtual ~Day08();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    vector<string> data;
};

#endif // DAY08_HPP
