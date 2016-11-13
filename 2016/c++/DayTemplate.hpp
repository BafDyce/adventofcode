#ifndef DAYTEMPLATE_H
#define DAYTEMPLATE_H

#include <iostream>
#include <vector>

#include "AoC.hpp"

using namespace std;

// Every day-specific class should inherit from this Template class
class DayTemplate {
    public:
        DayTemplate(vector<string> input);
        virtual ~DayTemplate();
        Result solve(int part);

    protected:
        // members
        vector<string> data;

        // we allow instantiation only with input
        // However, this constructor cannot be private due to child-classes
        DayTemplate();

        virtual Result solve_p1();
        virtual Result solve_p2();

    private:
};

#endif // DAYTEMPLATE_H
