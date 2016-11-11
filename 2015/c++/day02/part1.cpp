#include "Day02.h"

#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm> // min_element()

using namespace std;

unsigned int calc_paper_for_present(const string present);

bool Day02::solve_p1(string& result){
    unsigned int paper = 0;
    unsigned int length = data.size();
    for( unsigned int ii = 0; ii < length; ii++){
        paper += calc_paper_for_present(data[ii]);
    }

    result = to_string(paper);
    return true;
}

unsigned int calc_paper_for_present(const string present){
    // parsing input
    stringstream input = stringstream(present);
    unsigned int length;
    unsigned int width;
    unsigned int height;
    char x; // ununsed variable for eating the 'x's
    input >> length >> x >> width >> x >> height;

    // put areas into a vector, so that we can extract the minimum in one line
    vector<unsigned int> sides{length * width, length * height, width * height};
    unsigned int paper = 0;
    for(unsigned int ii = 0; ii < sides.size(); ii++){
        paper += (2 * sides[ii]);
    }

    // extra paper for each present: the area of the smallest side
    paper += *min_element(sides.begin(), sides.end());
    return paper;
}
