#include "BotInstruction.hpp"

#include <iostream>

using namespace std;

void BotInstruction::print(void){
    cout << chip << " from " << source
        << " -> " << targettype << " " << target << endl;
}
