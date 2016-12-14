#ifndef INSTRUCTION_HPP
#define INSTRUCTION_HPP

#include <vector>

using namespace std;

enum InstrType {
    COPY,
    INC,
    DEC,
    JNZ
};

struct Instruction {
    InstrType type;
    int source;
    bool source_is_reg;
    int target;

    void print(void);
    unsigned execute(vector<int> &registers);
};

#endif // INSTRUCTION_HPP
