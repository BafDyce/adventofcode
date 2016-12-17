#include "Day17.hpp"

#include <queue>

using namespace std;

Result Day17::solve_p1(){
    Field maze[MAZE_SIZE][MAZE_SIZE] = {
        {   {false, WALL, DOOR, DOOR, WALL},
            {false, WALL, DOOR, DOOR, DOOR},
            {false, WALL, DOOR, DOOR, DOOR},
            {false, WALL, WALL, DOOR, DOOR} },

        {   {false, DOOR, DOOR, DOOR, WALL},
            {false, DOOR, DOOR, DOOR, DOOR},
            {false, DOOR, DOOR, DOOR, DOOR},
            {false, DOOR, WALL, DOOR, DOOR} },

        {   {false, DOOR, DOOR, DOOR, WALL},
            {false, DOOR, DOOR, DOOR, DOOR},
            {false, DOOR, DOOR, DOOR, DOOR},
            {false, DOOR, WALL, DOOR, DOOR} },

        {   {false, DOOR, DOOR, WALL, WALL},
            {false, DOOR, DOOR, WALL, DOOR},
            {false, DOOR, DOOR, WALL, DOOR},
            {true, DOOR, DOOR, DOOR, DOOR} },
    };

    queue<Task> tasks;
    tasks.push({0, 0, ""});

    while( !tasks.empty() ){
        Task task = tasks.front();
        tasks.pop();

        Field *cell = &(maze[task.x][task.y]);

        if( cell->is_goal ){
            return {true, task.path};
        }

        string digest = compute_md5(data + task.path);
        // up
        if( cell->up == DOOR && is_open(digest[0]) ){
            tasks.push({task.x - 1, task.y, task.path + "U"});
        }

        // down
        if( cell->down == DOOR && is_open(digest[1]) ){
            tasks.push({task.x + 1, task.y, task.path + "D"});
        }

        // left
        if( cell->left == DOOR && is_open(digest[2]) ){
            tasks.push({task.x, task.y - 1, task.path + "L"});
        }

        // right
        if( cell->right == DOOR && is_open(digest[3]) ){
            tasks.push({task.x, task.y + 1, task.path + "R"});
        }
    }

    return {false, "No path found!"};
}
