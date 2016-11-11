#ifndef DAY03_H
#define DAY03_H

#include <../DayTemplate.h>

#include <iostream>
#include <vector>
//#include <functional>
//#include <iomanip>

using namespace std;

class Day03 : public DayTemplate {
    public:
        Day03(vector<string> input);
        virtual ~Day03();

    protected:
        string data;
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        Day03();
};

struct Position {
    int x;
    int y;

    void walk(const char direction){
        switch(direction){
        case '^':
            y++;
            break;
        case '>':
            x++;
            break;
        case 'v':
            y--;
            break;
        case '<':
            x--;
            break;
        default:
            // ignore invalid chars
            break;
        }
    }

    bool operator==(const Position &other) const {
        return x == other.x && y == other.y;
    }
};

namespace std {

  template <>
  struct hash<Position> {
    std::size_t operator()(const Position& pos) const {
      using std::size_t;
      using std::hash;
      using std::string;

      // Compute individual hash values for first,
      // second and third and combine them using XOR
      // and bit shifting:
      return ((hash<int>()(pos.x)
               ^ (hash<int>()(pos.y) << 1)) >> 1);
    }
  };

}

#endif // DAY03_H
