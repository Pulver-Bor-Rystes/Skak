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
    double get_time_passed_millis();

private:
    std::chrono::time_point<std::chrono::high_resolution_clock> start_time;
};

extern Timer timer;

extern bool stop_calculating;
extern bool use_time;
extern double stop_time;

static inline void check_if_time_is_up() {
    if(use_time && timer.get_time_passed_millis() > stop_time) {
        stop_calculating = true;
    }
}
