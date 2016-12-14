#include "Day14.hpp"

#include <vector>
#include <iostream>
#include <algorithm>
#include <map>

using namespace std;

Result Day14::solve_p1(){
    vector<Key> hits[16];
    map<int, bool> keys;

    for(unsigned int ii = 0; ii < numeric_limits<unsigned int>::max(); ii++){
        string hash = compute_md5(data + to_string(ii));

        vector<int> result;
        bool res = is_key(hits, hash, ii, result);
        if ( res ){
            for(int hit: result){
                if( keys.count(hit) == 0){
                    keys.insert({hit, true});
                    cout << "Found key " << keys.size()
                        <<  " at " << hit << endl;
                    if( keys.size() == 512 ){
                        cout << "Caluclated " << md5_computations
                            << " md5-hashes." << endl;
                        return {true, to_string(hit)};
                    }
                }
            }
        }
    }

    return {false, "Not enough keys found!"};
}


