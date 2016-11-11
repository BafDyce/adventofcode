#ifndef DAY01P2_H
#define DAY01P2_H

#include <iostream>
#include <vector>

using namespace std;

class Day01p2 {
    public:
        /** Default constructor */
        Day01p2(vector<string> input);
        /** Default destructor */
        virtual ~Day01p2();
        bool solve(string& result);

    protected:

    private:
        string data;
};

#endif // DAY01P2_H
