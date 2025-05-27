// place files you want to import through the `$lib` alias in this folder.

import init, { new_chessboard } from "chess_machine_lib";
import { board_id, c_did_init } from "./state";


export async function chess_init(): Promise<boolean> {
    return new Promise((resolve) => {
        c_did_init.subscribe(async v => {
            if (v) {
                resolve(true);
            }
            else {
                await init();
                v = true;
                resolve(true);
            }
        });
    });
}


export async function new_board(fen_str: string): Promise<number> {
    await chess_init();
    
    return new Promise(resolve => {
        board_id.subscribe(async v => {
            if (v == -1) {
                let new_id = new_chessboard(fen_str);
                board_id.set(new_id);
                resolve(new_id);
            }
            else {
                await init();
                
                let id = v;
                board_id.set(v);
                resolve(id);
            }
        });
    })
}