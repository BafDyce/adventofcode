#include "Day08.hpp"

#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

const int HEIGHT = 6;
const int WIDTH = 50;

static void setOn(bool grid[][WIDTH], int width, int height);
static void rotateRow(bool grid[][WIDTH], int row, int pixel);
static void rotateColumn(bool grid[][WIDTH], int colum, int pixel);
static void printGrid(bool grid[][WIDTH]);

Result Day08::solve_p2(){
    bool grid[HEIGHT][WIDTH] = {false};

    for(string line: this->data){
        if( line.find("rect") != std::string::npos ){
            int x, y;
            char tmp;
            stringstream stream(line.substr(5));
            stream >> x >> tmp >> y;
            setOn(grid, x, y);
        } else if( line.find("row") != std::string::npos ) {
            int row, pixel = 0;
            char a, b;
            stringstream stream(line.substr(13));
            stream >> row >> a >> b >> pixel;
            rotateRow(grid, row, pixel);
        } else if( line.find("column") != std::string::npos ) {
            int column, pixel = 0;
            char a, b;
            stringstream stream(line.substr(16));
            stream >> column >> a >> b >> pixel;
            rotateColumn(grid, column, pixel);
        }
    }

    printGrid(grid);
    return {true, "CHECK THE TERMINAL TO GET YOUR SOLUTION!!"};
}

static void setOn(bool grid[][WIDTH], int width, int height){
    for(int ii = 0; ii < width; ii++){
        for(int jj = 0; jj < height; jj++){
            grid[jj][ii] = true;
        }
    }
}

static void rotateRow(bool grid[][WIDTH], int prow, int pixel){
    bool row[WIDTH] = {false};

    for(int ii = 0; ii < WIDTH; ii++){
        row[ii] = grid[prow][ii];
    }

    for(int ii = 0; ii < WIDTH; ii++){
        grid[prow][(ii + pixel) % WIDTH] = row[ii];
    }
}

static void rotateColumn(bool grid[][WIDTH], int column, int pixel){
    bool col[HEIGHT] = {false};

    for(int ii = 0; ii < HEIGHT; ii++){
        col[ii] = grid[ii][column];
    }

    for(int ii = 0; ii < HEIGHT; ii++){
        grid[(ii + pixel) % HEIGHT][column] = col[ii];
    }
}

static void printGrid(bool grid[][WIDTH]){
    for(int ii = 0; ii < HEIGHT; ii++){
        for(int jj = 0; jj < WIDTH; jj++){
            cout << (grid[ii][jj] ? '#' : ' ' );
        }
        cout << endl;
    }
}
