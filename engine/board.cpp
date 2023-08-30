#include "board.h"
#include "utils.h"

U64 board::bitboards[12];

U64 board::occupancies[3];

// Side to move
int board::side = -1;

// En_passant square
int board::en_passant = no_sq;

// Castling rights
int board::castle = 0;

void board::parse_fen(string fen)
{
    memset(bitboards, 0ULL, sizeof(bitboards));

    side = 0;
    en_passant = no_sq;
    castle = 0;

    int i = 0;
    int square;

    for (int rank = 0; rank < 8; rank++)
    {
        for (int file = 0; file < 8; file++)
        {
            square = rank * 8 + file;

            if ((fen[i] >= 'A' && fen[i] <= 'Z') || (fen[i] >= 'a' && fen[i] <= 'z'))
            {
                set_bit(bitboards[char_pieces[fen[i]]], square);
            }

            else if (fen[i] >= '0' && fen[i] <= '9')
            {

                // difference in char values
                int offset = fen[i] - '0';
                // define piece variable
                int piece = -1;

                // loop over all piece bitboards
                for (int bb_piece = P; bb_piece <= k; bb_piece++)
                {
                    // if there is a piece on current square
                    if (is_occupied(bitboards[bb_piece], square))
                        // get piece code
                        piece = bb_piece;
                    break;
                }

                // on empty current square
                if (piece == -1)
                    // decrement file
                    file--;

                // adjust file counter
                file += offset;
            }

            else
            {
                file--;
            }

            i++;
        }
    }

    // side to mode
    i++;
    side = (fen[i] == 'w' ? white : black);

    // castling rights
    i += 2;
    while (fen[i] != ' ')
    {
        switch (fen[i])
        {
        case 'K':
            castle |= wk;
            break;
        case 'Q':
            castle |= wq;
            break;
        case 'k':
            castle |= bk;
            break;
        case 'q':
            castle |= bq;
            break;
        }
        i++;
    }

    // en passant square
    i++;
    if (fen[i] != '-')
    {
        int file = fen[i] - 'a';
        i++;
        int rank = 8 - (fen[i] - '0');

        en_passant = rank * 8 + file;
    }
    else
    {
        en_passant = no_sq;
    }

    update_occupancies();
}

int board::ply = 0;
long board::nodes = 0;

void board::search_position(int depth)
{
    stop_calculating = false;

    // Resets helper arrays
    memset(killer_moves, 0, sizeof(killer_moves));
    memset(history_moves, 0, sizeof(history_moves));
    memset(pv_length, 0, sizeof(pv_length));
    memset(pv_table, 0, sizeof(pv_table));

    int alpha = -50000;
    int beta = 50000;
    int candidate_pv_table_copy[246][246];
    int candidate_pv_length_copy[246];
    
    for (int current_depth = 1; current_depth <= depth; current_depth++)
    {
        if(stop_calculating) break;

        memcpy(&candidate_pv_table_copy, &pv_table, sizeof(pv_table));
        memcpy(&candidate_pv_length_copy, &pv_length, sizeof(pv_length));

        board::nodes = 0;

        int score = board::negamax(alpha, beta, current_depth);

        
        if(score <= alpha || score >= beta) {
            alpha = -50000;
            beta = 50000;
            --current_depth;
            continue;
        }

        alpha = score - bound_wiggle_room;
        alpha = score + bound_wiggle_room;
        if(!stop_calculating) {
            cout << "Found best move at depth " << current_depth << " looking through " << board::nodes << " nodes" << endl;
        }
        else {
            cout << "Interrupted by time at depth " << current_depth << " looking through " << board::nodes << " nodes" << endl;
        }
        cout << "Total time passed: " << timer.get_time_passed_millis() << " milliseconds." << endl;
        for (int i = 0; i < pv_length[0]; i++)
        {
            print::move(pv_table[0][i]);
            cout << " ";
        }
        cout << endl;
    }

    cout << "bestmove ";
    print::move(candidate_pv_table_copy[0][0]);
    cout << "\n\n";
}
