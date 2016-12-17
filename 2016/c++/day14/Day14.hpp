#ifndef DAY14_HPP
#define DAY14_HPP

#include <vector>
#include <iostream>

#include "../DayTemplate.hpp"

struct Key {
    string hash;
    char ch;
    int counter;
};

class Day14 : public DayTemplate {
public:
    Day14(vector<string>);
    virtual ~Day14();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;

    char get_first_tripple(string &hash);
    vector<char> get_quintuples(string &hash);
    bool is_key(vector<Key> hits[16], string &hash, int counter,
        vector<int> &result);

};

#endif // DAY14_HPP
