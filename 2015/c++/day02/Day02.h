#ifndef DAY02_H
#define DAY02_H

#include <iostream>
#include <vector>

#include "../DayTemplate.h"

using namespace std;

class Day02 : public DayTemplate {
    public:
        Day02(vector<string> input);
        virtual ~Day02();

    protected:
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        Day02();
};

#endif // DAY02_H
