use sisyphus32::{FenString, Search};

fn main() {
    let mut search = Search::default();
    search.go(&FenString::kiwipete().parse().unwrap(), Some(7), None);
}
