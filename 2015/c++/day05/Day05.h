#ifndef DAY05_H
#define DAY05_H

#include <../DayTemplate.h>

#include <iostream>
#include <vector>

class Day05 : public DayTemplate {
    public:
        Day05(vector<string> input);
        virtual ~Day05();

    protected:
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        Day05();
};

#endif // DAY05_H
