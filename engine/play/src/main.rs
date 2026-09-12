//! composes: verb-menu, interaction-resolver
//!
//! `cargo run -p play -- <game> [script]`
//!
//! With a script, plays it and prints the transcript. Without one, reads choices from stdin and
//! prints the menu before each — which is a text adventure with a list instead of a parser, and is
//! as much interface as a design with no `path-solver` and no `response-presenter` can honestly
//! offer. `.yidam/decisions/playable.yml`.

use play::{load, Session, Stop};

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = args.first() else {
        eprintln!("usage: play <game> [script]");
        return std::process::ExitCode::FAILURE;
    };
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{path}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let game = match load(&source, path) {
        Ok(g) => g,
        Err(problems) => {
            for p in problems {
                eprintln!("{path}: {p}");
            }
            return std::process::ExitCode::FAILURE;
        }
    };
    let mut session = match Session::start(&game) {
        Ok(s) => s,
        Err(stop) => {
            eprintln!("{stop:?}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let stop = match args.get(1) {
        Some(script) => {
            let text = match std::fs::read_to_string(script) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{script}: {e}");
                    return std::process::ExitCode::FAILURE;
                }
            };
            let lines: Vec<&str> = text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect();
            let stop = session.play(&lines);
            print!("{}", session.transcript());
            stop
        }
        None => interactive(&mut session),
    };
    println!("-- {stop:?}");
    std::process::ExitCode::SUCCESS
}

fn interactive(session: &mut Session) -> Stop {
    use std::io::Write;
    loop {
        let menu = session.menu();
        println!("\n{}", session.here());
        for (i, e) in menu.iter().enumerate() {
            println!("  {}. {e}", i + 1);
        }
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            return Stop::OutOfInput;
        }
        let line = line.trim();
        if line.is_empty() || line == "quit" {
            return Stop::OutOfInput;
        }
        let choice = match line.parse::<usize>() {
            Ok(n) if n >= 1 && n <= menu.len() => menu[n - 1].clone(),
            _ => line.to_string(),
        };
        match session.take(&choice) {
            Ok(stop) => {
                if let Some(t) = session.log.last() {
                    if let Some(line) = &t.said {
                        println!("  {line}");
                    }
                    if let Some(where_) = &t.moved {
                        println!("  → {where_}");
                    }
                    if t.silent() {
                        println!("  (nothing)");
                    }
                }
                if let Some(stop) = stop {
                    return stop;
                }
            }
            Err(stop) => return stop,
        }
    }
}
