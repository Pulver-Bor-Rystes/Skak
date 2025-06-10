use crate::info;

use super::*;


impl Actor for ServerThread {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Ready to handle ws sessions!");
        
        match EngineThread::new("engine/ChesslusPlus", ctx.address()) {
            Some(actor) => {
                self.engines.insert("juules".into(), actor.start());
            },
            None => {},
        };

        match EngineThread::new("stockfish/stockfish", ctx.address()) {
            Some(actor) => {
                self.engines.insert("stockfish".into(), actor.start());
            },
            None => {},
        };
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Server stopped");
    }
}