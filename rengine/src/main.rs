use rengine::Board;

fn main() {
    let mut b = Board::default();

    b.print();
    b.generate_moves_and_save();
    b.remove_pseudo_moves();

    println!("fen: {}", b.to_fen());
}
