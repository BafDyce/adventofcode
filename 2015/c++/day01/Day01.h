#ifndef DAY01_H
#define DAY01_H

#include <iostream>
#include <vector>

using namespace std;

class Day01 {
    public:
        /** Default constructor */
        Day01();
        Day01(vector<string> input);
        /** Default destructor */
        virtual ~Day01();
        bool solve(int part, string& result);

    protected:

    private:
        string data;
        bool solve_p1(string& result);
        bool solve_p2(string& result);
};

#endif // DAY01_H
