#ifndef OUTPUTBIN_HPP
#define OUTPUTBIN_HPP

class OutputBin {
public:
    OutputBin() : id(-1), chip(nullptr) {};
    OutputBin(int p_id) : id(p_id), chip(nullptr) {};

    bool feed_value(int value);
    int get_value(void);

private:
    int id;
    struct microchip *chip;
};

#endif // OUTPUTBIN_HPP
