#ifndef DAY01P1_H
#define DAY01P1_H

#include <iostream>
#include <vector>

using namespace std;

class Day01p1 {
    public:
        /** Default constructor */
        Day01p1();
        Day01p1(vector<string> input);
        /** Default destructor */
        virtual ~Day01p1();
        bool solve(string& result);

    protected:

    private:
        string data;
};

#endif // DAY01P1_H
