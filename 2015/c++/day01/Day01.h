#ifndef DAY01_H
#define DAY01_H

#include <iostream>
#include <vector>

#include "../DayTemplate.h"

using namespace std;

class Day01 : public DayTemplate {
    public:
        Day01(vector<string> input);
        virtual ~Day01();

    protected:
        string data;
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        Day01();

};

#endif // DAY01_H
