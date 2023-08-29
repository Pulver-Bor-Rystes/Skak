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
int board::best_move = 0;
long board::nodes = 0;

void board::search_position(int depth)
{
    Timer timer;

    // Resets helper arrays
    memset(killer_moves, 0, sizeof(killer_moves));
    memset(history_moves, 0, sizeof(history_moves));
    memset(pv_length, 0, sizeof(pv_length));
    memset(pv_table, 0, sizeof(pv_table));


    for (int current_depth = 1; current_depth <= depth; current_depth++)
    {
        board::nodes = 0;

        int score = board::negamax(-50000, 50000, current_depth);

        cout << "Found looking through " << board::nodes << " nodes" << endl;
        cout << "Task took: " << timer.get_time_passed() << " seconds." << endl;
        for (int i = 0; i < pv_length[0]; i++)
        {
            print::move(pv_table[0][i]);
            cout << " ";
        }
        cout << endl;
    }

    cout << "bestmove ";
    print::move(board::best_move);
    cout << "\n\n";
}
