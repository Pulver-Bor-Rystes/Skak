use actix::prelude::*;
use core::panic;
use std::{
    fmt::Display,
    io::{Read, Write}, // Traits skal være i brug!
    time::Duration,
};

pub struct Engine {
    name: String,
    handle: std::process::Child,
    response_over: ResponseOverAfter,
}

enum ResponseOverAfter {
    Str(String),
    Newlines(usize),
}

impl Engine {
    pub fn new(exe: &str) -> Engine {
        let handle = std::process::Command::new(exe)
            .current_dir("../")
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("fail");

        Engine {
            name: exe.to_string(),
            handle,
            response_over: ResponseOverAfter::Newlines(1),
        }
    }

    fn log(&self, output: impl Display) {
        println!("{} output: {}", self.name, output);
    }

    fn enable_uci(&mut self) {
        self.write("uci");
    }

    fn write(&mut self, msg: &str) {
        self.response_over = match msg {
            "uci" => ResponseOverAfter::Str("uciok".to_string()),
            "isready" => ResponseOverAfter::Str("readyok".to_string()),
            _ => ResponseOverAfter::Newlines(1),
        };

        if msg.contains("go") {
            self.response_over = ResponseOverAfter::Str("bestmove".to_string());
        }

        let input = self.handle.stdin.as_mut().unwrap();

        let txt = format!("{}\n", msg);
        // let txt = format!("{}", msg);
        // // input
        let _ = input.write_all(txt.as_bytes());
    }

    fn read(&mut self) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        let mut c = 0;
        loop {
            c += 1;

            let line = self.read_to_newline();
            output.push(line.clone());

            match &self.response_over {
                ResponseOverAfter::Str(str) => {
                    if line.contains(str) {
                        break;
                    }
                }
                ResponseOverAfter::Newlines(amount) => {
                    if &c >= amount {
                        break;
                    }
                }
            }
        }

        output
    }

    /// bruges sammen med read(&mut self)
    fn read_to_newline(&mut self) -> String {
        let mut line = String::new();

        match self.handle.stdout.as_mut() {
            Some(stdout) => loop {
                let mut buf = [0; 1];
                let _ = stdout.read(&mut buf);

                let str = String::from_utf8(buf.to_vec()).expect("noooo");
                line += &str;

                if str == "\n" {
                    break;
                }
            },
            None => {}
        }

        line
    }

    fn search(&mut self, position: String, max_time: std::time::Duration) -> String {
        self.write("ucinewgame");
        self.write(&format!("position fen {}", position));
        self.write(&format!("go movetime {}", max_time.as_millis()));

        println!(" >> engine ({}) is searching...\nwith fen string: {}", self.name, position);

        let output = self.read();
        let line = output.last();
        match line {
            Some(line) => {
                let bestmove = line.split_whitespace().nth(1).unwrap();
                println!(" >> best move found: {}", bestmove);
                return bestmove.to_string();
            }
            None => {}
        }

        panic!("no best move was found...");
    }
}

impl Actor for Engine {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.enable_uci();

        // let res = self.search("startpos".to_string(), Duration::from_secs(3));
        // println!("res: {}", res);
        // self.log(res);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!(" -> {} is stopping", self.name);

        Running::Stop
    }
}

#[derive(Message)]
#[rtype(result = "String")]
pub enum EngineAPI {
    /// 1. positionen, 2. maks tid i ms
    Search(String, Duration),
}

impl Handler<EngineAPI> for Engine {
    type Result = String;

    fn handle(&mut self, msg: EngineAPI, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            EngineAPI::Search(position, duration) => self.search(position, duration),
        }
    }
}
