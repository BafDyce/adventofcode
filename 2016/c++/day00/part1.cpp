#include "Day00.hpp"
#include "../AoC.hpp"

#include <iostream>
#include <algorithm> // reverse

using namespace std;

Result Day00::solve_p1(){
    string work = data;
    std::reverse(work.begin(), work.end());
    return {true, work};
}
