#include "Bot.hpp"

#include <iostream>

#include "Microchip.hpp"
#include "BotInstruction.hpp"

using namespace std;

int GLOBAL_ID = -1;

Bot::Bot() : id(-1), lower(nullptr), higher(nullptr) {}

Bot::~Bot(){
    if( this->lower ){
        delete this->lower;
    }

    if( this->higher ){
        delete this->higher;
    }

}

void Bot::set_id(int p_id){
    this->id = p_id;
}

int Bot::get_id(void){
    return this->id;
}

bool Bot::feed_value(int value){
    struct microchip *newchip = new microchip{value};
    bool check = this->feed_chip(newchip);
    if( !check ){
        cout << "ERROR: feeding failed!" << endl;
        delete newchip;
    }

    return check;
}

bool Bot::feed_chip(struct microchip *newchip){
    if( this->higher == nullptr && newchip ){
        if( this->lower == nullptr){
            this->lower = newchip;
        } else {
            if( (newchip->value == CMP_VAL_1
                    && this->lower->value == CMP_VAL_2)
                    || (newchip->value == CMP_VAL_2
                        && this->lower->value == CMP_VAL_1) ){
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

bool Bot::get_lower_value(int &value){
    if( this->lower ){
        value = this->lower->value;
        return true;
    }

    return false;
}

bool Bot::get_higher_value(int &value){
    if( this->higher ){
        value = this->higher->value;
        return true;
    }

    return false;
}

// actually unused but i'll keep it..
struct microchip * Bot::take_higher(void){
    struct microchip *retval = this->higher;
    this->higher = nullptr;
    return retval;
}

// actually unused but i'll keep it..
struct microchip * Bot::take_lower(void){
    struct microchip *retval = this->lower;
    this->lower = this->take_higher();
    return retval;
}

bool Bot::has_both_chips_set(void){
    return (this->lower && this->higher);
}

void Bot::print(void){
    cout << "id: " << id
    << " | lower: " << (lower ? lower->value : 0 )
    << " | higher: " << (higher ? higher->value : 0 )
    << endl;
}
