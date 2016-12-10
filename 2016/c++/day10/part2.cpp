#include "Day10.hpp"

#include <vector>
#include <iostream>
#include <sstream>
#include <queue>

#include "Bot.hpp"
#include "Microchip.hpp"
#include "BotInstruction.hpp"
#include "OutputBin.hpp"

using namespace std;

static const int BOTS = 300;
static const int OUTPUTS = 25;

Result Day10::solve_p2(){
    Bot bots[BOTS];
    OutputBin outputs[OUTPUTS];
    queue<BotInstruction> instructions;

    // =========== INIT ======
    for(int ii = 0; ii < BOTS; ii++){
        bots[ii].set_id(ii);
    }

    for(int ii = 0; ii < OUTPUTS; ii++){
        outputs[ii] = OutputBin(ii);
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
    while( !instructions.empty() ){
        BotInstruction instr = instructions.front();
        instructions.pop();

        Bot *bot = &(bots[instr.source]);
        if( bot->has_both_chips_set() ){
            int value = -1;
            if( (instr.chip == "low" && bot->get_lower_value(value))
                    || (instr.chip == "high" && bot->get_higher_value(value)) ){
                if( instr.targettype == "bot" ){
                    if( ! bots[instr.target].feed_value(value) ){
                        cout << "ERROR. this should not happen!" << endl;
                    }
                } else if( instr.targettype == "output" ){
                    if( ! outputs[instr.target].feed_value(value) ){
                        cout << "ERROR. this should not happen!" << endl;
                    }
                } else {
                    cout << "ERROR. This should never happen!!" << endl;
                }
            } else {
                cout << "ERROR. This should never happen!!" << endl;
            }

        }  else {
            instructions.push(instr);
        }
    }

    int result =
        outputs[0].get_value()
        * outputs[1].get_value()
        * outputs[2].get_value();


    return {true, to_string(result)};
}
