#ifndef ASSEMBUNNY_HPP
#define ASSEMBUNNY_HPP

#include <vector>
#include <iostream>

using namespace std;

enum InstrType {
    CPY,
    INC,
    DEC,
    JNZ,
    TGL
};

struct Instruction {
    unsigned id;
    InstrType type;
    int source;
    bool source_is_reg;
    int target;
    bool target_is_reg;

    void print(void);
    unsigned execute(vector<int> &registers);
    unsigned execute(vector<int> &registers, vector<Instruction> &instructions);
};

vector<Instruction> parse_assembunny(vector<string> assembunny);

#endif // ASSEMBUNNY_HPP
