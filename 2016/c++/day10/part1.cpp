#include "Day10.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <queue>

#include "Bot.hpp"
#include "Microchip.hpp"
#include "BotInstruction.hpp"

using namespace std;

static const int BOTS = 300;

Result Day10::solve_p1(){
    Bot bots[BOTS];
    queue<BotInstruction> instructions;

    // =========== INIT ======
    for(int ii = 0; ii < BOTS; ii++){
        bots[ii].set_id(ii);
    }

    for(string line: this->data){
        string var;
        stringstream stream(line);
        stream >> var;
        if( var == "bot" ){
            int botid = 0, targetid = 0;
            string gives, lowhigh, to, bot;
            stream >> botid >> gives >> lowhigh >> to >> bot >> targetid;

            BotInstruction instr {
                botid,
                bot,
                targetid,
                lowhigh
            };
            instructions.push(instr);

            string andword;
            stream >> andword >> lowhigh >> to >> bot >> targetid;

            BotInstruction instr2 {
                botid,
                bot,
                targetid,
                lowhigh
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
