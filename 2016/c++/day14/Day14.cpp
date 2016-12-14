#include "Day14.hpp"

#include <vector>
#include <iostream>
#include <ctime>
#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/md5.h>

Day14::Day14(vector<string> input)
        : data(input[0]), md5(nullptr), md5_computations(0)  {
    this->data = input[0];

    // init randomizer
    srand(time(nullptr));
    // setup openssl
    OpenSSL_add_all_digests();

    md5 = EVP_MD_CTX_create();
    EVP_DigestInit_ex(md5, EVP_md5(), nullptr);
}

Day14::~Day14() {
    EVP_MD_CTX_destroy(md5);
    EVP_cleanup();
}

string Day14::compute_md5(const string from) {
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

    ++md5_computations;
    return result;
}

string Day14::compute_stretched_md5(const string from) {
    string hash = from;
    for(unsigned ii = 0; ii <= 2016; ++ii){
        hash = compute_md5(hash);
    }

    return hash;
}

char Day14::get_first_tripple(string &hash){
    vector<char> last;
    last.push_back('\0');
    for(unsigned ii = 0; ii < 32; ++ii){
        char ch = hash[ii];
        if( ch != last[ last.size() - 1 ] ){
            last.erase(last.begin(), last.begin() + last.size());
            last.push_back(ch);
        } else {
            last.push_back(ch);
            if( last.size() == 3 ){
                return ch;
            }
        }
    }

    return '\0';
}

vector<char> Day14::get_quintuples(string &hash){
    vector<char> quintuples;
    vector<char> last;
    last.push_back('\0');
    for(unsigned ii = 0; ii < 32; ++ii){
        char ch = hash[ii];
        if( ch != last[ last.size() - 1 ] ){
            last.erase(last.begin(), last.begin() + last.size());
            last.push_back(ch);
        } else {
            last.push_back(ch);
            if( last.size() == 5 ){
                quintuples.push_back(ch);
                last.erase(last.begin(), last.begin() + last.size());
            }
        }
    }
    last.erase(last.begin(), last.begin() + 0);

    return quintuples;
}

bool Day14::is_key(vector<Key> hits[16], string &hash, int counter, vector<int> &result){
    vector<char> quintuple = get_quintuples(hash);
    bool retval = false;

    for(char ch: quintuple){
        unsigned idx = 0;
        if( ch >= '0' && ch <= '9'){
            idx = ch - '0';
        } else if ( ch >= 'a' && ch <= 'f'){
            idx = ch - 'a' + 10;
        } else {
            cerr << "ERROR! Wrong char: " << ch << endl;
        }

        for(Key key: hits[idx]){
            if(  key.counter >= (counter - 1000)){
                result.push_back(key.counter);
            }

            if( result.size() > 0 ){
                retval = true;
            }
        }
    }

    char ch = get_first_tripple(hash);
    if( ch != '\0' ){
        unsigned idx = 0;
        if( ch >= '0' && ch <= '9'){
            idx = ch - '0';
        } else if ( ch >= 'a' && ch <= 'f'){
            idx = ch - 'a' + 10;
        } else {
            cerr << "ERROR! Wrong char: " << ch << endl;
        }

        hits[idx].push_back(Key{hash, ch, counter});
    }

    return retval;
}

inline char Day14::get_hex_char(const unsigned char nibble) {
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
