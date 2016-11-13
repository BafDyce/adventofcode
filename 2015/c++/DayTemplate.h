#ifndef DAYTEMPLATE_H
#define DAYTEMPLATE_H

#include <iostream>
#include <vector>

using namespace std;

// Every day-specific class should inherit from this Template class
class DayTemplate {
    public:
        DayTemplate(vector<string> input);
        /** Default destructor */
        virtual ~DayTemplate();
        bool solve(int part, string& result);

    protected:
        // members
        vector<string> data;

        // we allow instantiation only with input
        DayTemplate();

        virtual bool solve_p1(string& result);
        virtual bool solve_p2(string& result);

    private:
};

#endif // DAYTEMPLATE_H
