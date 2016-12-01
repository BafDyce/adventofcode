#include "Day01.hpp"

#include <vector>
#include <sstream>
#include <iostream>

#include <boost/foreach.hpp>
#include <boost/tokenizer.hpp>

using namespace std;
using namespace boost;

Day01::Day01(vector<string> input) {
    char_separator<char> sep(", ");
    tokenizer<char_separator<char> > tokens(input[0], sep);
    BOOST_FOREACH(string t, tokens) {
        char direction = t[0];
        int distance;

        stringstream( t.substr(1) ) >> distance;
        Direction dir {direction, distance};

        this->data.push_back(dir);
    }
}

Day01::~Day01()
{
    //dtor
}
