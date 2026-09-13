//! composes: verb-menu, interaction-resolver
//!
//! `cargo run -p play -- <game> [script]`
//!
//! With a script, plays it and prints the transcript. Without one, reads choices from stdin and
//! prints the menu before each — which is a text adventure with a list instead of a parser.
//! `.yidam/decisions/playable.yml`.
//!
//! **The interactive loop holds the player now.** Its sink is [`play::Await`]: it writes the line
//! and does not come back until the player asks for the next thing, which is a hold performed by a
//! terminal with no clock and no invented rate. A script has no player, so a scripted run keeps
//! the sink that returns at once. `.yidam/decisions/dismissed-hold.yml`.

use play::{load, Await, Session, Stop};

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

    // One stream, and both the choice and the dismissal come through it. That is the constraint a
    // terminal has and a frame does not: nothing can listen for a player who has finished reading
    // while it is waiting for a player who has finished choosing.
    let stdin = std::io::stdin();
    let mut sink = Await::new(stdin.lock(), std::io::stdout());

    loop {
        let menu = session.menu();
        let read = session.reads();
        println!("\n{}", session.here());
        for (i, e) in read.iter().enumerate() {
            println!("  {}. {e}", i + 1);
        }
        print!("> ");
        let _ = std::io::stdout().flush();

        let Some(line) = sink.line() else {
            return Stop::OutOfInput;
        };
        if line.is_empty() || line == "quit" {
            return Stop::OutOfInput;
        }
        let choice = match line.parse::<usize>() {
            Ok(n) if n >= 1 && n <= menu.len() => menu[n - 1].clone(),
            _ => line.clone(),
        };
        // The lines are written by the sink, one hold each, so nothing is printed here that the
        // player has already been held for.
        match session.take_with(&choice, &mut sink) {
            Ok(stop) => {
                if let Some(t) = session.log.last() {
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
