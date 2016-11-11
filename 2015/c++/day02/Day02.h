#ifndef DAY02_H
#define DAY02_H

#include <iostream>
#include <vector>

using namespace std;

class Day02 {
    public:
        /** Default constructor */
        Day02();
        Day02(vector<string> input);
        /** Default destructor */
        virtual ~Day02();
        bool solve(int part, string& result);

    protected:

    private:
        vector<string> data;
        bool solve_p1(string& result);
        bool solve_p2(string& result);
};

#endif // DAY02_H
