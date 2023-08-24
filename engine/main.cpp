#include "shorthands.h"
#include "rng.h"
#include "movegen.h"
#include "perft.h"
#include "uci.h"
#include "utils.h"
#include "board.h"


int main()
{
    movegen::init();

    bool debugging = false;

    if (debugging)
    {
        board::parse_fen(tricky_position);
        print::game();

        board::search_position(1);

        moves move_list[1];
        board::generate_moves(move_list);
    }

    else
    {
        uci::init();
    }
    
    return 0;
}
