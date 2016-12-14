#include "Day13.hpp"

Result Day13::solve_p1(){

    int distance = find_path(Position{1, 1}, Position{31, 39});

    return {true, to_string(distance)};
}

int Day13::find_path(Position start, Position end){
    queue<Position> tasks;
    int visited[max_x][max_y];

    for(unsigned xx = 0; xx < max_x; ++xx){
        for(unsigned yy = 0; yy < max_y; ++yy){
            visited[xx][yy] = 0;
        }
    }

    tasks.push(start);

    while( !tasks.empty() ){
        Position pos = tasks.front();
        tasks.pop();

        if( pos.x == end.x && pos.y == end.y ){
            return visited[pos.x][pos.y];
        }

        // above
        if( pos.x > 0 && graph[pos.x - 1][pos.y]
                && visited[pos.x - 1][pos.y] == 0){
            visited[pos.x - 1][pos.y] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x - 1, pos.y});
        }

        // below
        if( pos.x < max_x - 1 && graph[pos.x + 1][pos.y]
                && visited[pos.x + 1][pos.y] == 0){
            visited[pos.x + 1][pos.y] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x + 1, pos.y});
        }

        // left
        if( pos.y > 0 && graph[pos.x][pos.y - 1]
                && visited[pos.x][pos.y - 1] == 0){
            visited[pos.x][pos.y - 1] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x, pos.y - 1});
        }

        // right
        if( pos.y < max_y - 1 && graph[pos.x][pos.y + 1]
                && visited[pos.x][pos.y + 1] == 0){
            visited[pos.x][pos.y + 1] = 1 + visited[pos.x][pos.y];
            tasks.push(Position{pos.x, pos.y + 1});
        }
    }

    return -1;
}
