#ifndef DAY17_HPP
#define DAY17_HPP

#include <iostream>
#include <vector>

#include <../DayTemplate.hpp>
#include <vector>
#include <string>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

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
    EVP_MD_CTX *md5;
    unsigned md5_computations;
    static unsigned const MAZE_SIZE = 4;

    Day17(const Day17 &);
    Day17 & operator=(const Day17 &);

    static bool is_open(char ch);

    string compute_md5(const string);
    string compute_stretched_md5(const string from);
    char get_hex_char(unsigned char nibble);
};

#endif // DAY17_HPP
