#include "Day05.hpp"

#include <vector>
#include <iostream>
#include <ctime>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

Day05::Day05(vector<string> input) : data(input[0]), md5(nullptr)  {
    this->data = input[0];

    // init randomizer
    srand(time(nullptr));
    // setup openssl
    OpenSSL_add_all_digests();

    md5 = EVP_MD_CTX_create();
    EVP_DigestInit_ex(md5, EVP_md5(), nullptr);
}

Day05::~Day05() {
    EVP_MD_CTX_destroy(md5);
    EVP_cleanup();
}

Result Day05::solve(const int part) {
    if( part == 3 ) {
        return this->solve_p3();
    }

    return DayTemplate::solve(part);
}

string Day05::compute_md5(const string from) {
    // init
    //EVP_DigestInit_ex(md5, EVP_md5(), nullptr);
    EVP_DigestInit(md5, EVP_md5());

    // compute hash
    unsigned char *buffer = (unsigned char *) from.c_str();
    EVP_DigestUpdate(md5, buffer, from.length());
    unsigned char hash_array[16];
    EVP_DigestFinal_ex(md5, hash_array, nullptr);

    string result = "";
    for(int ii = 0; ii < 16; ii++) {
        // about 3 times faster than using strnprintf()
        unsigned char byte = hash_array[ii];
        result += get_hex_char( (byte & 0xf0) >> 4 );
        result += get_hex_char(byte & 0xf);
    }

    return result;
}

inline char Day05::get_hex_char(const unsigned char nibble) {
    switch(nibble) {
    case 0x0:
        return '0';
    case 0x1:
        return '1';
    case 0x2:
        return '2';
    case 0x3:
        return '3';
    case 0x4:
        return '4';
    case 0x5:
        return '5';
    case 0x6:
        return '6';
    case 0x7:
        return '7';
    case 0x8:
        return '8';
    case 0x9:
        return '9';
    case 0xa:
        return 'a';
    case 0xb:
        return 'b';
    case 0xc:
        return 'c';
    case 0xd:
        return 'd';
    case 0xe:
        return 'e';
    case 0xf:
        return 'f';
    default:
        break;
    }

    cout << "Error in get_hex_char()";
    exit(1);
}
