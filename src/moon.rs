use std::fs;
use std::io;
use std::io::Write;

pub struct Moon {
    had_error: bool,
    had_runtime_error: bool,
}

impl Moon {
    pub fn new() -> Self {
        Moon { had_error: false, had_runtime_error: false }
    }

    pub fn run_file(&mut self, path: &String) {
        self.run(fs::read_to_string(path).unwrap(), true);
        if self.had_error {
            std::process::exit(65);
        }

        if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            let line = stdin.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    self.run(buffer.clone(), false);
                    self.had_error = false;
                }
                _ => break
            }
        }
    }

    pub fn run(&mut self, source: String, quit_on_error: bool) {
        todo!("implement run");
    }

    pub fn error(&mut self, line: u64, message: String) {
        self.report(line, String::from(""), message);
    }

    pub fn report(&mut self, line: u64, where_error: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_error, message);
        self.had_error = true;
    }
}
