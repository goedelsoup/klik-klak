use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: reachability <file.game>");
        return ExitCode::from(2);
    };
    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::from(2);
        }
    };
    let (game, mut problems) = game_format::parse(&source, &path);
    problems.extend(game_format::resolve(&game));
    if !problems.is_empty() {
        for p in &problems {
            eprintln!("{path}:{p}");
        }
        eprintln!("\n{path} does not parse cleanly; reachability is not computed over a game that does not resolve.");
        return ExitCode::FAILURE;
    }
    let report = reachability::analyse(&game);
    print!("{}", report.render());
    // A stranded state is a finding about the game, not a failure of this tool. The
    // specimen is deliberately trapped, so a nonzero exit here would make the fixture
    // permanently red; the caller decides what to do with the report.
    ExitCode::SUCCESS
}
