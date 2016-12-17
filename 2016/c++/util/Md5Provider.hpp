#ifndef AOC_MD5_HPP
#define AOC_MD5_HPP

#include <iostream>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

using namespace std;

class Md5Provider {
public:
    Md5Provider();
    virtual ~Md5Provider();

    string compute(const string);
    string compute_stretched(const string from, const unsigned iterations);
    unsigned get_computations(void);

protected:

private:
    static const unsigned DIGEST_SIZE = 16;
    EVP_MD_CTX *md5;
    unsigned computations;

    Md5Provider(const Md5Provider &);
    Md5Provider & operator=(const Md5Provider &);

    inline char get_hex_char(const unsigned char nibble);
};

#endif // AOC_MD5_HPP
