use fast_serv::{init_logger, start_server};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger(log::LevelFilter::Debug);
    start_server()
}
