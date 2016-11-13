#ifndef DAY04_H
#define DAY04_H

#include <iostream>
#include <vector>
#include <openssl/crypto.h>
#include <openssl/evp.h>

#include <../DayTemplate.h>

using namespace std;

class Day04 : public DayTemplate {
    public:
        Day04(vector<string> input);
        virtual ~Day04();

    protected:
        bool solve_p1(string& result);
        bool solve_p2(string& result);

    private:
        // puzzle input
        string data;
        EVP_MD_CTX *md5;

        Day04();
        string compute_md5(const string);
};

#endif // DAY04_H
