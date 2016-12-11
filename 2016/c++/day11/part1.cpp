#include "Day11.hpp"

// TODO: remove unnecessary includes!
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <map>

using namespace std;

enum ItemType {
    GENERATOR,
    CHIP
};

struct Item {
    ItemType type;
    string name;

    void print(void){
        string typestring = "n/a";
        switch(type){
        case GENERATOR:
            typestring = "generator";
            break;
        case CHIP:
            typestring = "chip";
            break;
        }
        cout << name << "-" << typestring;
    }

    bool operator<(const Item other) const {
        return this->type < other.type;
    }

    bool operator==(const Item other) const {
        return this->type == other.type
            && this->name == other.name;
    }
};

class Floor {
public:
    Floor() :
        name("n/a"),
        generators({}),
        chips({}),
        down(nullptr),
        up(nullptr) {
    };

    Floor(string name, vector<Item> items) : Floor() {
        this->name = name;
        for(Item item: items){
            _int_add_item(item);
        }
    }

    ~Floor(){};

    int move(int maxdepth){
        //cout << name << " can move:" << endl;

        if( maxdepth <= 0 ){
            if( is_finished() && up == nullptr){
                cout << name << ": FINISHED AT DEPTH " << maxdepth << endl;
                return 1;
            } else {
                return 9999;
            }
        }

        if( generators.size() == 0 && chips.size() == 0 ){
            cerr << "THIS SHOULD NOT HAPPEN!! .. in move()" << endl;
            return 9999;
        }

        if( up == nullptr
                && generators.size() == max_elements
                && chips.size() == max_elements){
            return 1;
        }

        vector<vector<Item>> takeaways = get_takeaways();

        vector<vector<Item>>::iterator it = takeaways.begin();
        vector<vector<Item>>::iterator itEnd = takeaways.end();

        /*for (; it != itEnd; ++it) {
            for(Item item: *it){
                item.print();
                cout << " and ";
            }
            cout << endl;
        }

        cout << endl;*/

        int best = 9999;
        for(vector<Item> elevator: takeaways){
            if( this->up && this->up->add_items(elevator) ){
                this->remove_items(elevator);
                int res = this->up->move(maxdepth - 1);
                this->up->remove_items(elevator);
                this->add_items(elevator);
                if ( res < best ){
                    best = res;
                    cout << "Found new best: " << best << endl;
                }
            } else if( this->down && this->down->add_items(elevator) ){
                this->remove_items(elevator);
                int res = this->down->move(maxdepth - 1);
                this->down->remove_items(elevator);
                this->add_items(elevator);
                if ( res < best ){
                    best = res;
                    cout << "Found new best: " << best << endl;
                }
            }
        }

        return best + 1;
    }

    void connect_upper(Floor *upper){
        up = upper;
        upper->set_lower(this);
    }

    void set_upper(Floor *upper){
        up = upper;
    }

    void set_lower(Floor *lower){
        down = lower;
    }

    bool add_items(vector<Item> newItems){
        if( newItems.size() == 0 || newItems.size() > 2 ){
            return false;
        }

        map<string, bool> generators_bak(generators);
        map<string, bool> chips_bak(chips);
        for(Item item: newItems){
            _int_add_item(item);
        }

        bool retval = is_valid();
        if( !retval ){
            generators = generators_bak;
            chips = chips_bak;
        }

        return retval;
    }

    void remove_items(vector<Item> remItems){
        for(Item item: remItems){
            switch(item.type){
            case ItemType::GENERATOR:
                generators.erase(item.name);
                break;
            case ItemType::CHIP:
                chips.erase(item.name);
                break;
            }
        }
    }

    void print(void){
        cout << name << ":" << endl;
        cout << "Generators (" << generators.size() << "): ";
        for(auto gen: generators){
            cout << gen.first << ", ";
        }
        cout << endl;

        cout << "Chips (" << chips.size() << "): ";
        for(auto chip: chips){
            cout << chip.first << ", ";
        }
        cout << endl << "valid: " << (is_valid() ? "yes" : "no") << endl;
        cout << "down: " << down << " | up: " << up << endl;
        cout << "==========================" << endl;
    }

    void print_down(void){
        print();
        if ( down ){
            down->print_down();
        }
    }

private:
    string name;
    map<string, bool> generators;
    map<string, bool> chips;
    const int max_elements = 5;
    Floor *down;
    Floor *up;

    vector<vector<Item>> get_takeaways(void){
        vector<vector<Item>> takeaways;

        map<string, bool> generators_bak = generators;
        map<string, bool> chips_bak = chips;
        // single items
        for(auto chip: chips){
            takeaways.push_back( {Item{ItemType::CHIP, chip.first}} );
        }

        for(auto gen: generators){
            if( generators.size() > 1 ){
                // not the only generator: check if our chip is here
                if( !has_chip(gen.first) ){
                    takeaways.push_back({Item{ItemType::GENERATOR, gen.first}});
                }
            }
        }

        // pairs
        for(auto chip: chips){
            string chipname = chip.first;

            // chip & chip
            for(auto chip2: chips){
                if( chipname != chip2.first ){
                    takeaways.push_back({
                        Item{ItemType::CHIP, chipname},
                        Item{ItemType::CHIP, chip2.first}
                    });
                }
            }
        }

        // gen
        for(auto gen: generators){
            string gen_name = gen.first;

            // gen & gen
            if( has_chip(gen_name) ){
                // only allowed to remove another gen if there are exactly 2
                if( generators.size() == 2 ){
                    // gen & gen special case
                    for(auto gen2: generators){
                        if( gen2.first != gen_name ){
                            takeaways.push_back({
                                Item{ItemType::GENERATOR, gen_name},
                                Item{ItemType::GENERATOR, gen2.first}
                            });
                        }
                    }
                }
            } else {
                // allowed to remove anything
                // so take any chip
                // gen & chip
                for(auto chip: chips){
                    takeaways.push_back({
                        Item{ItemType::GENERATOR, gen_name},
                        Item{ItemType::CHIP, chip.first}
                    });
                }

                // also, take any gen
                if( generators.size() > 1 ){
                    // as long as they dont leave their chip with a third gen
                    if( generators.size() == 2 ){
                        // gen & gen special case
                        for(auto gen2: generators){
                            if( gen2.first != gen_name ){
                                takeaways.push_back({
                                    Item{ItemType::GENERATOR, gen_name},
                                    Item{ItemType::GENERATOR, gen2.first}
                                });
                            }
                        }
                    } else {
                        // 3 or more gens
                        for(auto gen2: generators){
                            if( gen_name != gen2.first &&
                                    !has_chip(gen2.first) ){
                                takeaways.push_back({
                                    Item{ItemType::GENERATOR, gen_name},
                                    Item{ItemType::GENERATOR, gen2.first}
                                });
                            }
                        }
                    }
                }
            }
        }

        // remove duplicates
        //  find them
        vector<int> dups;
        for(unsigned ii = 0; ii < takeaways.size() - 1; ++ii){
            vector<Item> a = takeaways[ii];
            vector<Item> b = takeaways[ii + 1];

            if(a.size() != b.size() ){
                continue;
            }

            int same = 0;
            for(Item item: a){
                if( find(b.begin(), b.end(), item) != b.end() ){
                    ++same;
                }
            }

            if( same == a.size() ){
                dups.push_back(ii);
            }
        }

        //  actually remove them
        vector<int>::reverse_iterator rit = dups.rbegin();
        for (; rit != dups.rend(); ++rit){
            int idx = *rit;
            takeaways.erase(takeaways.begin() + idx);
        }

        return takeaways;
    }

    bool has_chip(string name){
        for(auto chip: chips){
            if( chip.first == name ){
                return true;
            }
        }

        return false;
    }

    bool has_generator(string name){
        for(auto gen: generators){
            if( gen.first == name ){
                return true;
            }
        }

        return false;
    }

    bool is_valid(void){
        if( chips.size() == 0 || generators.size() == 0 ){
            return true;
        }

        // at this point we know that we have at least one chip and one gen
        if( chips.size() > generators.size() ){
            // at least one chip is not protected
            return false;
        }

        // check if each chip has a corresponding generator
        int matching = 0;
        for(auto chip: chips){
            for(auto gen: generators){
                if( chip == gen ){
                    ++matching;
                    break;
                }
            }
        }

        return matching == chips.size();
    }

    void _int_add_item(Item item){
        switch(item.type){
        case ItemType::GENERATOR:
            generators[item.name] = true;
            break;
        case ItemType::CHIP:
            chips[item.name] = true;
            break;
        default:
            cerr << "SHOULD NOT HAPPEN! _int_add_item()" << endl;
        }
    }

    bool is_finished(void){
        if( generators.size() == max_elements && chips.size() == max_elements ){
            return true;
        }
    }
};

Result Day11::solve_p1(){

    Item strontium_gen{GENERATOR, "strontium"};
    Item strontium_chip{CHIP, "strontium"};
    Item plutonium_gen{GENERATOR, "plutonium"};
    Item plutonium_chip{CHIP, "plutonium"};
    Item thulium_gen{GENERATOR, "thulium"};
    Item thulium_chip{CHIP, "thulium"};
    Item ruthenium_gen{GENERATOR, "ruthenium"};
    Item ruthenium_chip{CHIP, "ruthenium"};
    Item curium_gen{GENERATOR, "curium"};
    Item curium_chip{CHIP, "curium"};

    vector<Item> v1;
    v1.push_back(strontium_gen);
    v1.push_back(strontium_chip);
    v1.push_back(plutonium_gen);
    v1.push_back(plutonium_chip);

    vector<Item> v2;
    v2.push_back(thulium_gen);
    v2.push_back(ruthenium_gen);
    v2.push_back(ruthenium_chip);
    v2.push_back(curium_gen);
    v2.push_back(curium_chip);

    vector<Item> v3;
    v3.push_back(thulium_chip);

    Floor f1("Floor 1", v1);
    Floor f2("Floor 2", v2);
    Floor f3("Floor 3", v3);
    Floor f4("Floor 4", {});

    f1.connect_upper(&f2);
    f2.connect_upper(&f3);
    f3.connect_upper(&f4);

    f4.print_down();

    cout << "RUNNING" << endl;
    int result = f1.move(40);
    cout << "GLOBAL RESULT floor 1: " << result << endl;
    result = f2.move(40);
    cout << "GLOBAL RESULT floor 2: " << result << endl;
    result = f3.move(40);
    cout << "GLOBAL RESULT floor 3: " << result << endl;

    return {true, to_string(result)};
}
