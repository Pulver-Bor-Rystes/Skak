# Sisyphus32
Sisyphus32 is an open-source pruning-based chess engine!
It can be run as an [executable](https://github.com/Juules32/sisyphus32/releases), where it uses UCI for communication.
Alternatively, it can be used as a [crate](https://crates.io/crates/sisyphus32).

# [Lichess Bot](https://lichess.org/@/Sisyphus32)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=rapid)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=blitz)
![](https://lichess-shield.vercel.app/api?username=sisyphus32&format=bullet)

# Supported UCI Commands ([documentation](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html))
Sisyphus32 is UCI-compliant, implementing the following UCI commands:
- `uci`
- `ucinewgame`
- `isready`
- `position (fen <fenstring> | startpos) [moves <move1> ... <movei>]`
- `go [perft <plies> | [depth <plies>] [wtime <ms>] [btime <ms>] [winc <ms>] [binc <ms>] [movetime <ms>]]`
- `stop | s`
- `quit | q`
- `exit | e`
- `eval`
- `display | d`
- `bench | benchmedium`
- `benchlong`
- `benchshort`
- `setoption name Clear Hash`
- `setoption name Threads value <n>`
- `setoption name SyzygyPath value <path>`
- `setoption name Hash value <size_mb>`

# Local Development

## Setup
1. Install Rust.
2. (OPTIONAL) download a syzygy tablebase and put it in `tables/syzygy/` for optimal performance.

## How to use
1. Run `cargo build --release` to build the strongest version of the engine.
2. Run `cargo run --release` to build and run the strongest version of the engine.
3. Run `cargo run --release --no-default-features --features <version>` to build and run a specific version of the engine. Version names can be found in `src/versions.rs`.
4. Run `cargo test -- --test-threads=1` to run all unit and integration tests.
6. Run `cargo run --bin test_all` to run all unit and integration tests for all versions.
5. (WINDOWS ONLY) Run `cargo run --bin build_all` to build executables for all versions to `target/release_all/`.
7. (WINDOWS ONLY) Run `cargo run --bin cutechess_sprt <version1> <version2>` to run SPRT against the specified versions, which should correspond to binary names in `target/release_all/`. This requires [Cute Chess](https://github.com/cutechess/cutechess) to be installed.
8. (WINDOWS ONLY) Run `cargo run --bin samply_profile <profile name>` to run a profiler on the specified profile name, which should correspond to a file in `src/bin/`. This requires [Samply](https://github.com/mstange/samply) to be installed.
