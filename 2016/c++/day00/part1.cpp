#include "Day00.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <limits>
#include <queue>

#include "../util/Md5Provider.hpp"

using namespace std;

struct Task {
    int n;
    int steps;
};

Result Day00::solve_p1(){
    Md5Provider md5;
    string result = "";

    // iterate over input lines
    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        result += md5.compute(var);
    }
    // END iterate over input lines

    // BFS
    queue<Task> tasks;
    tasks.push({17, 0});
    bool visited[2000];
    visited[17] = true;

    while( !tasks.empty() ){
        Task task = tasks.front();
        tasks.pop();

        if( task.n == 1337 ){
            return {true, to_string(task.steps)};
            // might be also needed sometimes
            continue;
        } else if( task.n > 1337 ){
            continue;
        }

        for(unsigned ii = 0; ii < 10; ++ii){
            int x = task.n + ii;
            if( visited[x] == false ){
                tasks.push({x, task.steps + 1});
            }
        }
    }
    return {false, "Nothing found!"};
    // END BFS

    return {true, result};
}
