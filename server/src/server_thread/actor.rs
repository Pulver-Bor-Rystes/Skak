use std::{error::Error, process::Stdio};

use crate::{error, info};

use super::*;

impl Actor for ServerThread {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.start_bots(ctx).expect("Noget gik galt");

        info!("Ready to handle ws sessions!");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Server stopped");
    }
}

