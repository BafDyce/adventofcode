#include "Day07.hpp"

#include <vector>
#include <iostream>
#include <deque>

using namespace std;

bool supports_TLS(string ipv7);

Result Day07::solve_p1(){

    int counter = 0;
    for(string line: this->data){
        if( supports_TLS(line) ){
            ++counter;
        }
    }

    return {true, to_string(counter)};
}


bool supports_TLS(string ipv7){
    bool result = false;
    bool in_hypernet_seq = false;
    // saves just the last 3 letters
    deque<char> last;

    for(char ch: ipv7){
        if(ch == '[' && ! in_hypernet_seq){
            in_hypernet_seq = true;
            // we changed from supernet to hypernet sequence
            // -> reset last chars
            while( last.size() > 0 ){
                last.pop_front();
            }
        } else if( ch == ']' && in_hypernet_seq){
            in_hypernet_seq = false;
            // we changed from hypernet to supernet sequence
            // -> reset last chars
            while( last.size() > 0 ){
                last.pop_front();
            }
        } else {
            // fill up queue
            if( last.size() < 3 ){
                last.push_back(ch);
                continue;
            }

            if( ch == last[0] && last[1] == last[2] && ch != last[1]){
                if( in_hypernet_seq ){
                    return false;
                }
                result = true;
            }

            last.push_back(ch);
            last.pop_front();
        }
    }

    return result;
}
