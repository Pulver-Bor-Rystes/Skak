use super::*;


impl Actor for ServerThread {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Ready to handle ws sessions!");

        self.engines.insert(
            "juules".to_string(),
            EngineThread::new("engine/ChessPlusPlus", ctx.address()).start(),
        );

        self.engines.insert(
            "stockfish".to_string(),
            EngineThread::new("stockfish/stockfish", ctx.address()).start(),
        );
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Server stopped");
    }
}