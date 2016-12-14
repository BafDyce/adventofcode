#include "OutputBin.hpp"

#include "Microchip.hpp"

bool OutputBin::feed_value(int value){
    if( this->chip == nullptr ){
        this->chip = new microchip{value};
        return true;
    }

    return false;
}

int OutputBin::get_value(void){
    if( this->chip ){
        return this->chip->value;
    }

    return 0;
}
