#ifndef DAY13_HPP
#define DAY13_HPP

#include <iostream>
#include <vector>
#include <string>
#include <queue>

#include "../DayTemplate.hpp"

struct Position {
    unsigned x;
    unsigned y;
};

class Day13 : public DayTemplate {
public:
    Day13(vector<string> input);
    virtual ~Day13();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    int designers_number;
    static const unsigned max_x = 45;
    static const unsigned max_y = 45;
    bool graph[max_x][max_y];

    bool is_open_space(const int x, const int y);
    void build_graph();
    int find_path(Position start, Position end);
    int count_reachable_tiles(Position start, int max_distance);

};

#endif // DAY13_HPP
