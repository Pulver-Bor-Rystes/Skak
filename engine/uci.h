#pragma once
#include "shorthands.h"

/*
    The uci namespace is used for defining necessary
    universal chess interface functions    
*/

namespace uci {
    void init();
    void loop();
    void parse_position(string input);
    void parse_moves(string input);
    int parse_move(string move_string);
    void parse_go(string input);
    void parse_json(string input);
    void print_engine_info();
}