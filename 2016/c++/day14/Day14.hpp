#ifndef DAY14_HPP
#define DAY14_HPP

#include <../DayTemplate.hpp>

#include <vector>
#include <iostream>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

struct Key {
    string hash;
    char ch;
    int counter;
};

class Day14 : public DayTemplate {
public:
    Day14(vector<string>);
    virtual ~Day14();

protected:
    Result solve_p1();
    Result solve_p2();

private:
    string data;
    EVP_MD_CTX *md5;
    unsigned md5_computations;

    Day14(const Day14 &);
    Day14 & operator=(const Day14 &);

    string compute_md5(const string);
    string compute_stretched_md5(const string from);
    char get_hex_char(unsigned char nibble);

    char get_first_tripple(string &hash);
    vector<char> get_quintuples(string &hash);
    bool is_key(vector<Key> hits[16], string &hash, int counter,
        vector<int> &result);

};

#endif // DAY14_HPP
