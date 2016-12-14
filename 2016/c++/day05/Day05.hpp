#ifndef DAY05_HPP
#define DAY05_HPP

#include <../DayTemplate.hpp>

#include <vector>
#include <iostream>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

class Day05 : public DayTemplate
{
    public:
        Day05(vector<string>);
        virtual ~Day05();
        Result solve(const int part);

    protected:
        Result solve_p1();
        Result solve_p2();
        Result solve_p3();

    private:
        string data;
        EVP_MD_CTX *md5;

        Day05(const Day05 &);
        Day05 & operator=(const Day05 &);

        string compute_md5(const string);
        char get_hex_char(unsigned char nibble);
};

#endif // DAY05_HPP
