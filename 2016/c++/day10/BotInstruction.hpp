#ifndef BOTINSTRUCTION_HPP_INCLUDED
#define BOTINSTRUCTION_HPP_INCLUDED

#include <string>

using namespace std;

struct BotInstruction {
    int source;
    string targettype; // "bot" or "output"
    int target;
    string chip; // "high" or "low"

    BotInstruction() :
        source(0),
        targettype("undefined"),
        target(0),
        chip("undefined"){
    };

    BotInstruction(int p_source, string p_targettype, int p_target,
            string p_chip) :
        source(p_source),
        targettype(p_targettype),
        target(p_target),
        chip(p_chip){
    };

    ~BotInstruction(){};

    void print(void);
};

#endif // BOTINSTRUCTION_HPP_INCLUDED
