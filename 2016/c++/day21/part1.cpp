#include "Day21.hpp"

#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

Result Day21::solve_p1(){
    string password = "abcdefgh";
    //string password = "abcde";

    return {true, scramble_pw(password)};
}

string Day21::scramble_pw(string &pw){
    string password = pw;

    // iterate over input lines
    for(string line: this->data){
        if( line.find("swap position") != std::string::npos ){
            stringstream instr(line.substr(13));
            int idx_start, idx_end;
            string with, position;
            instr >> idx_start >> with >> position >> idx_end;

            char tmp = password[idx_start];
            password[idx_start] = password[idx_end];
            password[idx_end] = tmp;
        } else if( line.find("swap letter") != std::string::npos ){
            stringstream instr(line.substr(11));
            char ch1, ch2;
            string with, letter;
            instr >> ch1 >> with >> letter >> ch2;

            for(unsigned ii = 0; ii < password.length(); ++ii){
                if( password[ii] == ch1 ){
                    password[ii] = ch2;
                } else if( password[ii] == ch2 ){
                    password[ii] = ch1;
                }
            }
        } else if( line.find("rotate ") != std::string::npos ){
            stringstream instr(line.substr(7));
            string direction;
            instr >> direction;

            if( direction == "left" || direction == "right" ){
                int steps;
                instr >> steps;

                if( steps == 0){
                    continue;
                }

                string pw2 = "";
                unsigned idx = 0;
                if( direction == "right" ){
                    idx = password.length() - steps;
                } else {
                    idx = steps;
                }
                while( pw2.length() != password.length() ){
                    pw2 += password[idx];
                    idx = (idx + 1) % password.length();
                }
                password = pw2;
            } else if( direction == "based" ){
                string on, position, of, letter;
                char ch;
                instr >> on >> position >> of >> letter >> ch;

                int ii = 0;
                while( password[ii] != ch ){
                    ++ii;
                }

                ++ii;
                if( ii >= 5 ){
                    ++ii;
                }
                ii %= password.length();

                if( ii == 0){
                    continue;
                }

                int idx = password.length() - ii;
                string pw2 = "";
                while( pw2.length() != password.length() ){
                    pw2 += password[idx];
                    idx = (idx + 1) % password.length();
                }
                password = pw2;
            } else {
                cerr << "ERROR!!!" << direction << endl;
            }

        } else if( line.find("reverse position") != std::string::npos ){
            stringstream instr(line.substr(18));
            unsigned idx_start, idx_end;
            string through;
            instr >> idx_start >> through >> idx_end;

            string pw2 = "";
            unsigned idx = 0;
            while( idx < idx_start ){
                pw2 += password[idx];
                ++idx;
            }

            string reversed = "";
            while( idx <= idx_end ){
                reversed += password[idx];
                ++idx;
            }

            std::reverse(reversed.begin(), reversed.end());
            pw2 += reversed;

            while(idx < password.length()){
                pw2 += password[idx];
                ++idx;
            }

            password = pw2;

        } else if( line.find("move position") != std::string::npos ){
            stringstream instr(line.substr(14));
            unsigned x, y = 0;
            string to, position;

            instr >> x >> to >> position >> y;

            char tmp = password[x];
            string pw2 = "";
            unsigned idx1 = 0, idx2 = 0;
            //cout << "\t";
            while( pw2.length() < password.length() ){
                if( idx2 == y ){
                    pw2 += tmp;
                    ++idx2;
                    if( pw2.length() == password.length() ){
                        break;
                    }
                }

                if( idx1 == x ){
                    ++idx1;
                }

                pw2 += password[idx1];
                ++idx2;
                ++idx1;
            }

            password = pw2;

        }
    }
    // END iterate over input lines

    return password;
}
