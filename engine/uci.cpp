#include "uci.h"
#include "board.h"
#include "utils.h"
#include "perft.h"
#include <sstream>
#include <vector>
#include "libs/json.hpp"

void uci::init()
{
    uci::print_engine_info();
    uci::loop();
}

void uci::print_engine_info()
{
    cout << "id name ChessPlusPlus" << endl;
    cout << "id author Juules32" << endl;
    cout << "uciok" << endl;
}

void uci::loop()
{
    string input;
    while (true)
    {
        getline(cin, input);
        
        if (input == "quit" || input == "exit")
        {
            break; // Exit the loop if the user enters "quit"
        }

        else if (input == "uci")
        {
            uci::print_engine_info();
        }

        else if (input == "isready")
        {
            cout << "readyok" << endl;
        }

        else if (input == "ucinewgame")
        {
            uci::parse_position("position startpos");
        }

        uci::parse_position(input);
        uci::parse_go(input);
        uci::parse_json(input);
    }
}


void uci::parse_json(string input)
{
    using json = nlohmann::json;
    size_t json_i = input.find("json");

    if (json_i != string::npos)
    {
        size_t movedata_i = input.find("movedata");
        if (movedata_i != string::npos)
        {
            moves move_list[1];
            board::generate_moves(move_list);

            json json_moves_data = json::array();

            for (int i = 0; i < move_list->size; i++)
            {
                int move = move_list->array[i];
                json move_data;

                move_data["source_square"] = index_to_square[get_source(move)];
                move_data["target_square"] = index_to_square[get_target(move)];
                move_data["piece_type"] = string(1, ascii_pieces[get_piece(move)]);
                move_data["promotion_piece_type"] = string(1, ascii_pieces[get_promotion_piece_type(move)]);
                move_data["is_capture"] = is_capture(move) ? true : false;
                move_data["is_double_pawn_push"] = is_double_pawn_push(move) ? true : false;
                move_data["is_en_passant"] = is_en_passant(move) ? true : false;
                move_data["is_castling"] = is_castling(move) ? true : false;

                json_moves_data.push_back(move_data);
            }

            cout << json_moves_data << endl;
        }
    }
}

void uci::parse_position(string input)
{
    size_t position_i = input.find("position");

    if (position_i != string::npos)
    {
        size_t startpos_i = input.find("startpos");
        size_t trickypos_i = input.find("trickypos");
        size_t killerpos_i = input.find("killerpos");
        size_t cmkpos_i = input.find("cmkpos");
        size_t rookpos_i = input.find("rookpos");
        size_t fen_i = input.find("fen");
        size_t moves_i = input.find("moves");
        // Get position
        if (startpos_i != string::npos)
        {
            board::parse_fen(start_position);
        }
        else if(trickypos_i != string::npos) 
        {
            board::parse_fen(tricky_position);
        }
        else if(killerpos_i != string::npos) 
        {
            board::parse_fen(killer_position);
        }
        else if(cmkpos_i != string::npos) 
        {
            board::parse_fen(cmk_position);
        }
        else if(rookpos_i != string::npos) 
        {
            board::parse_fen(rook_position);
        }
        else if (fen_i != string::npos)
        {
            board::parse_fen(input.substr(fen_i + 4));
        }

        // Make moves if specified
        if (moves_i != string::npos)
        {
            uci::parse_moves(input.substr(moves_i + 6));
        }

        print::game();
    }
}

void uci::parse_go(string input)
{
    size_t go_i = input.find("go");

    if (go_i != string::npos)
    {
        size_t depth_i = input.find("depth");
        size_t perft_i = input.find("perft");
        size_t eval_i = input.find("eval");
        size_t wtime_i = input.find("wtime");
        size_t btime_i = input.find("btime");
        size_t winc_i = input.find("winc");
        size_t binc_i = input.find("binc");
        int depth = 6;
        int inc = -1;
        int time = -1;
        int moves_to_go = 30;
        use_time = false;
        stop_time = numeric_limits<double>::infinity();

        if (depth_i != string::npos)
        {
            depth = stoi(input.substr(depth_i + 6));
        }
        else if (perft_i != string::npos)
        {
            // String to integer
            perft::test(stoi(input.substr(perft_i + 6)));
            return;
        }
        else if (eval_i != string::npos)
        {
            // String to integer
            cout << board::eval() << endl;
            return;
        }

        if (wtime_i != string::npos && board::side == white)
        {
            time = stoi(input.substr(wtime_i + 6));
        }
        if (btime_i != string::npos && board::side == black)
        {
            time = stoi(input.substr(btime_i + 6));
        }
        if (winc_i != string::npos && board::side == white)
        {
            inc = stoi(input.substr(winc_i + 5));
        }
        if (binc_i != string::npos && board::side == black)
        {
            inc = stoi(input.substr(binc_i + 5));
        }

        if(time != -1) {
            timer.reset();
            use_time = true;

            // - 100 is a small offset to counteract the
            // inevitable delay after stop_time is set to true
            stop_time = time / moves_to_go - 100 + inc;

            board::search_position(64);
        }

        else {
            board::search_position(depth);
        }
    }
}

void uci::parse_moves(string input)
{

    // Creates a stringstream from the input string
    stringstream ss(input);

    // Uses a vector to store the substrings
    vector<string> substrings;

    string substring;

    // Extracts substrings separated by space and stores them in the vector
    while (ss >> substring)
        substrings.push_back(substring);

    for (const string &str : substrings)
    {
        if (uci::parse_move(str))
            board::make_move(uci::parse_move(str));
    }
}

int uci::parse_move(string move_string)
{

    int source_square = move_string[0] - 'a' + (8 - (move_string[1] - '0')) * 8;
    int target_square = move_string[2] - 'a' + (8 - (move_string[3] - '0')) * 8;

    moves move_list[1];

    board::generate_moves(move_list);

    for (int move_count = 0; move_count < move_list->size; move_count++)
    {
        int current_move = move_list->array[move_count];
        if (source_square == get_source(current_move) && target_square == get_target(current_move))
        {
            int promotion_piece_type = get_promotion_piece_type(current_move) % 6;
            if (!promotion_piece_type)
                return current_move;

            switch (move_string[4])
            {
            case 'q':
                if (promotion_piece_type == Q)
                    return current_move;
                else
                    return 0;
                break;

            case 'r':
                if (promotion_piece_type == R)
                    return current_move;
                else
                    return 0;
                break;

            case 'b':
                if (promotion_piece_type == B)
                    return current_move;
                else
                    return 0;
                break;

            case 'n':
                if (promotion_piece_type == N)
                    return current_move;
                else
                    return 0;
                break;

            default:
                return 0;
                break;
            }
        }
    }

    return 0;
}