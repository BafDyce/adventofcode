#include "Md5Provider.hpp"

#include <iostream>
#include <ctime>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

using namespace std;

Md5Provider::Md5Provider() : md5(nullptr), computations(0) {
    // init randomizer
    srand(time(nullptr));
    // setup openssl
    OpenSSL_add_all_digests();

    md5 = EVP_MD_CTX_create();
    EVP_DigestInit_ex(md5, EVP_md5(), nullptr);
}

Md5Provider::~Md5Provider() {
    EVP_MD_CTX_destroy(md5);
    EVP_cleanup();
}

string Md5Provider::compute(const string from) {
    // init
    //EVP_DigestInit_ex(md5, EVP_md5(), nullptr);
    EVP_DigestInit(md5, EVP_md5());

    // compute hash
    unsigned char *buffer = (unsigned char *) from.c_str();
    EVP_DigestUpdate(md5, buffer, from.length());
    unsigned char hash_array[DIGEST_SIZE];
    EVP_DigestFinal_ex(md5, hash_array, nullptr);

    string result = "";
    for(int ii = 0; ii < DIGEST_SIZE; ii++) {
        // about 3 times faster than using strnprintf()
        unsigned char byte = hash_array[ii];
        result += get_hex_char( (byte & 0xf0) >> 4 );
        result += get_hex_char(byte & 0xf);
    }

    ++computations;
    return result;
}

string Md5Provider::compute_stretched(
        const string from, const unsigned iterations){
    string hash = from;
    for(unsigned ii = 0; ii < iterations; ++ii){
        hash = this->compute(hash);
    }

    return hash;
}

unsigned Md5Provider::get_computations(void){
    return computations;
}


inline char Md5Provider::get_hex_char(const unsigned char nibble) {
    switch(nibble) {
    case 0x0: return '0';
    case 0x1: return '1';
    case 0x2: return '2';
    case 0x3: return '3';
    case 0x4: return '4';
    case 0x5: return '5';
    case 0x6: return '6';
    case 0x7: return '7';
    case 0x8: return '8';
    case 0x9: return '9';
    case 0xa: return 'a';
    case 0xb: return 'b';
    case 0xc: return 'c';
    case 0xd: return 'd';
    case 0xe: return 'e';
    case 0xf: return 'f';
    default: break;
    }

    cerr << "Error in get_hex_char()" << endl;
    exit(1);
}
