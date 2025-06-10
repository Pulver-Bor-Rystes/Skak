pub const FEATURES: &[&str] = &[
    "iterative_deepening",
    "minimax",
    "negamax",
    "pst",
    "sort_moves",
    "quiescence",
    "quiescence_en_passant",
    "checks_add_depth",
    "killer_heuristic",
    "history_heuristic",
    "tt",
    "eval_tt",
    "tt_two_tier",
    "null_move_pruning",
    "late_move_reductions",
    "tapered_eval",
    "positional_eval",
    "pseudo_pins",
    "capture_with_check_eval",
    "move_flag_eval",
    "lazy_smp",
    "opening_book",
    "syzygy_tablebase",
];

pub const BASE_FEATURES: &[&str] = &[
    "base_basic",
    "base_magic_number",
    "base_clone",
    "base_clone_parallel",
    "base_array",
    "base_array_parallel",
];

pub const OTHER_FEATURES: &[&str] = &[
    "bot_game",
];
