#ifndef DAY17_HPP
#define DAY17_HPP

#include <iostream>
#include <vector>

#include "../DayTemplate.hpp"
#include <vector>
#include <string>

enum Wall {
    WALL,
    DOOR,
};

struct Field {
    bool is_goal;
    Wall up;
    Wall right;
    Wall down;
    Wall left;
};

struct Task {
    unsigned x;
    unsigned y;
    string path;
};

class Day17 : public DayTemplate {
public:
    Day17(vector<string> input);
    virtual ~Day17();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;
    static unsigned const MAZE_SIZE = 4;

    static bool is_open(char ch);
};

#endif // DAY17_HPP
