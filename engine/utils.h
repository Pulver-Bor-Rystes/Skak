#pragma once
#include "shorthands.h"

/*
    The print namespace contains useful functions
    for printing various chess elements
*/

namespace print {
    void move(int move);
    void all_moves(moves *move_list);
    void bitboard(U64 bitboard);
    void game();
    void attacked_squares(int side);
}

class Timer {
public:
    Timer();
    void reset();
    double get_time_passed();

private:
    std::chrono::time_point<std::chrono::high_resolution_clock> start_time;
};