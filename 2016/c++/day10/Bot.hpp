#ifndef BOT_HPP
#define BOT_HPP

#include "Microchip.hpp"
#include "BotInstruction.hpp"

const int CMP_VAL_1 = 61;
const int CMP_VAL_2 = 17;

extern int GLOBAL_ID;

class Bot {
public:
    Bot();
    ~Bot();

    void set_id(int p_id);
    int get_id(void);
    bool feed_value(int value);
    bool feed_chip(struct microchip *newchip);
    bool get_lower_value(int &value);
    bool get_higher_value(int &value);
    // actually unused but i'll keep it..
    struct microchip *take_higher(void);
    // actually unused but i'll keep it..
    struct microchip *take_lower(void);
    bool has_both_chips_set(void);
    void print(void);

private:
    int id;
    struct microchip *lower;
    struct microchip *higher;

    // disabling copying of bots
    Bot(const Bot& bot);
    Bot & operator=(const Bot &);
};

#endif // BOT_HPP
