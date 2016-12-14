#include "Day13.hpp"


Result Day13::solve_p2(){

    int tiles = count_reachable_tiles(Position{1, 1}, 50);

    return {true, to_string(tiles)};
}

int Day13::count_reachable_tiles(Position start, int max_distance){
    queue<Position> tasks;
    int visited[max_x][max_y];

    for(unsigned xx = 0; xx < max_x; ++xx){
        for(unsigned yy = 0; yy < max_y; ++yy){
            visited[xx][yy] = -1;
        }
    }

    tasks.push(start);
    visited[start.x][start.y] = 0;

    int count = 0;
    while( !tasks.empty() ){
        Position pos = tasks.front();
        tasks.pop();

        if( visited[pos.x][pos.y] > max_distance ){
            continue;
        } else {
            ++count;
        }

        // above
        if( pos.x > 0 && graph[pos.x - 1][pos.y]
                && visited[pos.x - 1][pos.y] == -1){
            visited[pos.x - 1][pos.y] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x - 1, pos.y});
        }

        // below
        if( pos.x < max_x - 1 && graph[pos.x + 1][pos.y]
                && visited[pos.x + 1][pos.y] == -1){
            visited[pos.x + 1][pos.y] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x + 1, pos.y});
        }

        // left
        if( pos.y > 0 && graph[pos.x][pos.y - 1]
                && visited[pos.x][pos.y - 1] == -1){
            visited[pos.x][pos.y - 1] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x, pos.y - 1});
        }

        // right
        if( pos.y < max_y - 1 && graph[pos.x][pos.y + 1]
                && visited[pos.x][pos.y + 1] == -1){
            visited[pos.x][pos.y + 1] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x, pos.y + 1});
        }
    }

    return count;
}
