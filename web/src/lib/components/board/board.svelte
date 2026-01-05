<script lang="ts">
    import { onMount } from "svelte";
    import { socket } from "$lib/ws";
    import { chessboard_oneshot_init, username, type Game } from "$lib/state";
    import { new_chessboard, get_pieces, get_move_name, load_fen } from "chess_machine_lib";
    let sg: Game = { hash: "", white: "", black: "", current_fen: "", move_history: [""] };

    let { game = sg } = $props();
    let opponent = $username == game.white ? game.black : game.white;
    let flip = false;

    let selected = -1;

    let turn = game.current_fen.split(" ")[1] == "b" ? false : true;

    let board_id: number = -1;
    // Define the size of the canvas and the chessboard
    const canvasWidth = 400;
    const canvasHeight = 400;
    const squareSize = 50;

    type MouseInfo = {
        index: number;
        x: number;
        y: number;
    };

    type Piece = {
        x: number;
        y: number;
        image: HTMLImageElement | null;
    };
    let p_on_board: Piece[] = [];

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    onMount(async () => {
        // fire the oneshot init fn
        await chessboard_oneshot_init();

        if (board_id == -1) {
            board_id = new_chessboard(game.current_fen);
        }

        // Initialize the canvas
        ctx = canvas.getContext("2d");
        canvas.width = canvasWidth;
        canvas.height = canvasHeight;
        draw_chessboard_background();
        load_pieces();
        draw_pieces();

        canvas.addEventListener("mousedown", drag_start);
        canvas.addEventListener("mousemove", drag);
        canvas.addEventListener("mouseup", drag_exit);
    });

    $socket.on(`game:${game.hash}:update`, (result, content) => {
        console.log("Game update!");
        game = content;

        load_fen(board_id, game.current_fen);
        load_pieces();
        draw_pieces();
    });

    // Draw the chessboard
    function draw_chessboard_background() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                const x = col * squareSize;
                const y = row * squareSize;
                const color = (row + col) % 2 === 0 ? "white" : "black";
                ctx.fillStyle = color;
                ctx.fillRect(x, y, squareSize, squareSize);
            }
        }
    }

    function load_pieces() {
        p_on_board = [];
        let pieces = get_pieces(board_id);

        let is_black = game.black == $username;
        is_black = flip ? !is_black : is_black;

        let index = is_black ? 63 : 0;

        for (let row = 1; row <= 8; row++) {
            for (let col = 1; col <= 8; col++) {
                const piece_name = pieces[index];
                const x = col * squareSize + squareSize / 2;
                const y = row * squareSize + squareSize / 2;

                if (piece_name != "*") {
                    const image = new Image();
                    image.src = `/static/${piece_name}.png`;

                    let piece: Piece = { image, x, y };
                    image.onload = () => draw_p(piece);
                    p_on_board.push(piece);
                } else {
                    p_on_board.push({ image: null, x, y });
                }

                index += is_black ? -1 : 1;
            }
        }
    }

    function calc_index(start: number): number {
        let i = 0;
        while (start > squareSize) {
            start -= squareSize;
            i += 1;
        }
        return i;
    }

    function calc_mouse_info(e: DragEvent): MouseInfo {
        let bx = canvas.getBoundingClientRect();
        let x = calc_index(e.clientX - bx.x);
        let y = calc_index(e.clientY - bx.y);

        return {
            index: y * 8 + x,
            x: e.clientX - bx.x,
            y: e.clientY - bx.y,
        };
    }

    function drag_start(e: DragEvent) {
        if (selected >= 0) return;

        selected = calc_mouse_info(e).index;

        console.log(p_on_board[selected]);

        if (p_on_board[selected].image == null) {
            selected = -1;
        }
    }

    function drag(e: DragEvent) {
        if (selected == -1) return;

        let mouse = calc_mouse_info(e);
        p_on_board[selected].x = mouse.x + squareSize;
        p_on_board[selected].y = mouse.y + squareSize;

        draw_pieces();
    }

    function drag_exit(e: DragEvent) {
        let mouse = calc_mouse_info(e);

        let r = 0; // remainder
        let y = 0; // y position

        let from = selected;
        let to = mouse.index;

        // grimt :)) men for det til at virke!
        if (game.black == $username) {
            from = 63 - from;
            to = 63 - to;
        }
        if (flip) {
            from = 63 - from;
            to = 63 - to;
        }

        // hvis det er træk, lav det og beregn forfra
        let move_name = get_move_name(board_id, from, to);
        if (move_name == "") {
            r = selected % 8;
            y = (selected - r) / 8;
        } else {
            r = mouse.index % 8;
            y = (mouse.index - r) / 8;
        }

        p_on_board[selected].x = r * squareSize + 75;
        p_on_board[selected].y = y * squareSize + 75;
        draw_pieces();

        selected = -1;

        $socket.send("game:move", {
            hash: game.hash,
            move_name,
        });
    }

    function draw_pieces() {
        draw_chessboard_background();

        let index = -1;
        for (let piece of p_on_board) {
            if (++index == selected) continue;

            // drag piece
            draw_p(piece);
        }

        // draw selected piece
        if (selected != -1) {
            draw_p(p_on_board[selected]);
        }
    }

    function draw_p(piece: Piece) {
        if (piece.image == null) return;
        ctx.drawImage(
            piece.image,
            piece.x - piece.image.width / 2,
            piece.y - piece.image.height / 2,
            squareSize,
            squareSize,
        );
    }
</script>

<!-- In svelte, make a chessboard via canvas -->

<div class="m-5 p-5 flex">
    <div>
        <p>{opponent}</p>

        <canvas bind:this={canvas}> </canvas>
        <p>{$username}</p>
    </div>
    <div>
        <p><b>hash:</b> {game.hash}</p>
        <p><b>turn:</b> {turn}</p>
    </div>
</div>
