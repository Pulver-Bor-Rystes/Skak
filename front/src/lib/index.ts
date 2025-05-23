// place files you want to import through the `$lib` alias in this folder.

import init, { greet } from "chess_machine_lib";


export async function greet_test() {
    await init();
    console.log(greet("World"));
}