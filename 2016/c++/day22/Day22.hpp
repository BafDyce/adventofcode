#ifndef DAY22_HPP
#define DAY22_HPP

#include <iostream>
#include <vector>
#include <string>

#include "../DayTemplate.hpp"

struct Node {
    int size;
    int used;

    int avail(void){
        return size - used;
    }

    void print(void){
        cout << used << "/" << size << endl;
    }
};

class Day22 : public DayTemplate {
public:
    Day22(vector<string> input);
    virtual ~Day22();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    static const int gridsize_x = 35;
    static const int gridsize_y = 29;
    Node grid[gridsize_x][gridsize_y];
};

#endif // DAY22_HPP
