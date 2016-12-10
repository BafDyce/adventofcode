#include "Day10.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <queue>

using namespace std;

static const int BOTS = 300;

static const int CMP_VAL_1 = 61;
static const int CMP_VAL_2 = 17;
// example1.input
//static const int CMP_VAL_1 = 2;
//static const int CMP_VAL_2 = 3;
static int GLOBAL_ID = -1;

struct microchip {
    int value;
};

class Bot {
public:
    Bot(){};

    Bot(int id){
        this->id = id;
        this->lower = nullptr;
        this->higher = nullptr;
    }

    ~Bot(){
        if( this->lower ){
            delete this->lower;
        }

        if( this->higher ){
            delete this->higher;
        }

    }

    int get_id(void){
        return this->id;
    }

    bool feed_value(int value){
        struct microchip *newchip = new microchip{value};
        bool check = this->feed_chip(newchip);
        if( !check ){
            cout << "ERROR: feeding failed!" << endl;
            delete newchip;
        }

        return check;
    }

    bool feed_chip(struct microchip *newchip){
        if( this->higher == nullptr && newchip ){
            if( this->lower == nullptr){
                this->lower = newchip;
            } else {
                if( (newchip->value == CMP_VAL_1 && this->lower->value == CMP_VAL_2)
                        || (newchip->value == CMP_VAL_2 && this->lower->value == CMP_VAL_1) ){
                    cout << "IT IS BOT " << this->id << endl;
                    GLOBAL_ID = this->id;
                }

                if( newchip->value < this->lower->value ){
                    this->higher = this->lower;
                    this->lower = newchip;
                } else {
                    this->higher = newchip;
                }
            }

            return true;
        }

        return false;
    }

    bool get_lower_value(int &value){
        if( this->lower ){
            value = this->lower->value;
            return true;
        }

        return false;
    }

    bool get_higher_value(int &value){
        if( this->higher ){
            value = this->higher->value;
            return true;
        }

        return false;
    }

    // actually unused but i'll keep it..
    struct microchip *take_higher(void){
        struct microchip *retval = this->higher;
        this->higher = nullptr;
        return retval;
    }

    // actually unused but i'll keep it..
    struct microchip *take_lower(void){
        struct microchip *retval = this->lower;
        this->lower = this->take_higher();
        return retval;
    }

    bool has_both_chips_set(void){
        return (this->lower && this->higher);
    }

    void print(void){
        cout << "id: " << id
        << " | lower: " << (lower ? lower->value : 0 )
        << " | higher: " << (higher ? higher->value : 0 )
        << endl;
    }

private:
    int id;
    struct microchip *lower;
    struct microchip *higher;

    // disabling copying of bots
    Bot(const Bot& bot);
};

struct BotInstruction {
    int source;
    string targettype; // "bot" or "output"
    int target;
    string chip; // "high" or "low"

    void print(void){
        cout << chip << " from " << source
            << " -> " << targettype << " " << target << endl;
    }
};

Result Day10::solve_p1(){
    Bot bots[BOTS];
    queue<BotInstruction> instructions;

    // =========== INIT ======
    for(int ii = 0; ii < BOTS; ii++){
        bots[ii] = Bot(ii);
    }

    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        if( var == "bot" ){
            unsigned botid = 0, targetid = 0;
            string gives, lowhigh, to, bot;
            stream >> botid >> gives >> lowhigh >> to >> bot >> targetid;

            BotInstruction instr {
                .source = botid,
                .targettype = bot,
                .target = targetid,
                .chip = lowhigh
            };
            instructions.push(instr);

            string andword;
            stream >> andword >> lowhigh >> to >> bot >> targetid;

            BotInstruction instr2 {
                .source = botid,
                .targettype = bot,
                .target = targetid,
                .chip = lowhigh
            };
            instructions.push(instr2);

        } else if( var == "value" ){
            int value, botid;
            string goes, to, bot;
            stream >> value >> goes >> to >> bot >> botid;

            bots[botid].feed_value(value);
        }
    }

    // ==== PERFORM INSTRUCTIONS ====
    while( !instructions.empty() && GLOBAL_ID == -1 ){
        BotInstruction instr = instructions.front();
        instructions.pop();

        Bot *bot = &(bots[instr.source]);
        if( bot->has_both_chips_set()
                && instr.targettype == "bot" ){
            int value = -1;
            if( (instr.chip == "low" && bot->get_lower_value(value))
                    || (instr.chip == "high" && bot->get_higher_value(value)) ){
                if( ! bots[instr.target].feed_value(value) ){
                    cout << "ERROR. this should not happen!" << endl;
                }
            } else {
                cout << "ERROR. This should never happen!!" << endl;
            }
        } else {
            instructions.push(instr);
        }
    }


    return {true, to_string(GLOBAL_ID)};
}
