use actix::Addr;

use crate::{info, server_thread::ServerThread};

use super::{types::ResponseOverAfter, EngineThread};
use std::{fmt::Display, io::{Read, Write}};



impl EngineThread {
    pub fn new(exe: &str, server_addr: Addr<ServerThread>) -> EngineThread {
        let handle = std::process::Command::new(exe)
            .current_dir("../")
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("fail");

        EngineThread {
            server_addr,
            name: exe.to_string(),
            handle,
            response_over: ResponseOverAfter::Newlines(1),
        }
    }

    fn log(&self, output: impl Display) {
        info!("{} output: {}", self.name, output);
    }

    pub fn enable_uci(&mut self) {
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

    pub fn search(&mut self, position: String, max_time: std::time::Duration) -> String {
        self.write("ucinewgame");
        self.write(&format!("position fen {}", position));
        self.write(&format!("go movetime {}", max_time.as_millis()));

        info!(" >> engine ({}) is searching...\nwith fen string: {}", self.name, position);

        let output = self.read();
        let line = output.last();
        match line {
            Some(line) => {
                let bestmove = line.split_whitespace().nth(1).unwrap();
                info!(" >> best move found: {}", bestmove);
                return bestmove.to_string();
            }
            None => {}
        }

        panic!("no best move was found...");
    }
}
