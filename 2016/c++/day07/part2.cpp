#include "Day07.hpp"

#include <vector>
#include <iostream>
#include <queue>

using namespace std;

bool supports_SSL(string ipv7);
pair<vector<string>, vector<string>> get_abas_n_babs(string ipv7);

Result Day07::solve_p2(){

    int counter = 0;

    for(string line: this->data){
        if( supports_SSL(line) ){
            ++counter;
        }
    }

    return {true, to_string(counter)};
}

bool supports_SSL(string ipv7){
    pair<vector<string>, vector<string>> sequences = get_abas_n_babs(ipv7);

    for(string aba: sequences.first){
        string check = "";
        check += aba[1];
        check += aba[0];
        check += aba[1];
        for(string bab: sequences.second){
            if( bab == check){
                return true;
            }
        }
    }

    return false;
}

pair<vector<string>, vector<string>> get_abas_n_babs(string ipv7){
    vector<string> abas;
    vector<string> babs;
    bool in_hypernet_seq = false;
    // saves the last 2 characters
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
            if( last.size() < 2 ){
                last.push_back(ch);
                continue;
            }

            if( ch == last[0] && ch != last[1]){
                if( in_hypernet_seq ){
                    string bab = "";
                    bab += ch;
                    bab += last[1];
                    bab += ch;
                    babs.push_back(bab);
                } else {
                    string aba = "";
                    aba += ch;
                    aba += last[1];
                    aba += ch;
                    abas.push_back(aba);
                }
            }

            last.push_back(ch);
            last.pop_front();
        }
    }

    return make_pair(abas, babs);
}
