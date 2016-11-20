#ifndef DAY06_H
#define DAY06_H

#include <../DayTemplate.h>

#include <vector>
#include <iostream>
#include <sstream>  // stringstream
#include <iomanip>  // setw()

enum Action {
    ON,
    OFF,
    TOGGLE,
    NONE
};

struct Day6Position {
    unsigned x;
    unsigned y;
};

struct Instruction {
    Action action;
    Day6Position start;
    Day6Position end;

    Instruction(string inp){
        unsigned idx = 0;
        if( inp.find("turn on") != string::npos ){
            action = ON;
            idx = 7;
        } else if( inp.find("turn off") != string::npos ){
            action = OFF;
            idx = 8;
        } else if( inp.find("toggle") != string::npos ){
            action = TOGGLE;
            idx = 6;
        } else {
            action = NONE;
        }

        stringstream ss( inp.substr(idx) );
        unsigned x1;
        unsigned y1;
        unsigned x2;
        unsigned y2;
        char comma;
        string through;

        ss >> x1 >> comma >> y1
            >> setw(9) >> through
            >> x2 >> comma >> y2;
        start = Day6Position{x1, y1};
        end = Day6Position{x2, y2};
    }

    void print(){
        cout << action << " | "
            << start.x << "," << start.y << " | "
            << end.x << "," << end.y << endl;
    }
};

class Day06 : public DayTemplate {
    public:
        Day06(vector<string> input);
        virtual ~Day06();

    protected:

    private:
        vector<Instruction> instructions;
        static const int gridsize = 1000;
        int grid[gridsize][gridsize] = {0};

        Day06();
        bool solve_p1(string& result);
        bool solve_p2(string& result);

        int count_lights();
        void turn_on(Day6Position from, Day6Position to);
        void turn_off(Day6Position from, Day6Position to);
        void toggle(Day6Position from, Day6Position to);
        void brighten(Day6Position from, Day6Position to, int value = 1);
        void dim(Day6Position from, Day6Position to);

};

#endif // DAY06_H
