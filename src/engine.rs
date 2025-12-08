use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub struct Engine {
    process: std::process::Child,
}

impl Engine {
    pub fn new() -> Self {
        let process = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut engine = Engine { process };
        engine.send("uci");
        engine.readUntil("uciok");
        engine
    }

    pub fn send(&mut self, cmd: &str) {
        let stdin = self.process.stdin.as_mut().unwrap();
        stdin.write_all(cmd.as_bytes()).unwrap();
        stdin.write_all(b"\n").unwrap();
        stdin.flush().unwrap();
    }

    pub fn readLine(&mut self) -> String {
        let stdout = self.process.stdout.as_mut().unwrap();
        let mut reader = BufReader::new(stdout);
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        buf
    }

    pub fn readUntil(&mut self, token: &str) {
        loop {
            let line = self.readLine();
            if line.contains(token) {
                break;
            }
        }
    }

    pub fn bestMove(&mut self, moves: &str) -> Option<String> {
        self.send(&format!("position startpos moves {}", moves));
        self.send("go depth 12");

        loop {
            let line = self.readLine();
            if line.starts_with("bestmove") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                } else {
                    return None;
                }
            }
        }
    }
}
