use std::process::ExitCode;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("games") => {
            print_games();
            ExitCode::SUCCESS
        }
        Some("chess") => run_chess(&args[1..]),
        _ => {
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn run_chess(rest: &[String]) -> ExitCode {
    match rest.first().map(|s| s.as_str()) {
        Some("verify") => chess_verify(),
        Some("perft") => chess_perft(&rest[1..]),
        Some("moves") => chess_moves(&rest[1..]),
        _ => {
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn row(a: &str, b: &str, c: &str) {
    println!("{a:<12} {b:<10} {c}");
}

fn print_games() {
    row("GAME", "STATUS", "NOTES");
    row("----", "------", "-----");
    row("chess", "ready", "full move generation, perft, benchmarks");
    row("backgammon", "planned", "not yet implemented");
    row("checkers", "planned", "not yet implemented");
    row("go", "planned", "not yet implemented");
}

fn print_usage() {
    eprintln!("usage: parlor <games|chess>");
    eprintln!("  games");
    eprintln!("  chess verify");
    eprintln!("  chess perft [--fen <FEN>] [--depth N] [--divide]");
    eprintln!("  chess moves [--fen <FEN>]");
}

fn chess_verify() -> ExitCode {
    let mut all_ok = true;
    for benchmark in parlor_chess::BENCHMARKS {
        let check = parlor_chess::check(benchmark);
        let passed = check.passed();
        if !passed {
            all_ok = false;
        }
        let status = if passed { "PASS" } else { "FAIL" };
        println!(
            "[{status}] {} depth {} expected {} observed {}",
            check.benchmark.name,
            check.benchmark.depth,
            check.benchmark.expected_nodes,
            check.observed_nodes
        );
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn chess_perft(rest: &[String]) -> ExitCode {
    let mut fen = START_FEN.to_string();
    let mut depth: u32 = 1;
    let mut divide = false;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--fen" => {
                if i + 1 < rest.len() {
                    fen = rest[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--fen requires a value");
                    return ExitCode::FAILURE;
                }
            }
            "--depth" => {
                if i + 1 < rest.len() {
                    match rest[i + 1].parse::<u32>() {
                        Ok(d) => {
                            depth = d;
                            i += 2;
                        }
                        Err(_) => {
                            eprintln!("--depth requires an integer");
                            return ExitCode::FAILURE;
                        }
                    }
                } else {
                    eprintln!("--depth requires a value");
                    return ExitCode::FAILURE;
                }
            }
            "--divide" => {
                divide = true;
                i += 1;
            }
            other => {
                eprintln!("unknown argument: {other}");
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    if divide {
        let mut total = 0u64;
        for (mv, nodes) in board.perft_divide(depth) {
            println!("{}: {nodes}", parlor_chess::move_name(&mv));
            total += nodes;
        }
        println!("total: {total}");
    } else {
        println!("{}", board.perft(depth));
    }
    ExitCode::SUCCESS
}

fn chess_moves(rest: &[String]) -> ExitCode {
    let mut fen = START_FEN.to_string();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--fen" => {
                if i + 1 < rest.len() {
                    fen = rest[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--fen requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown argument: {other}");
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    let moves = board.legal_moves();
    println!("{} legal moves", moves.len());
    for mv in &moves {
        println!("{}", parlor_chess::move_name(mv));
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn start_position_has_twenty_moves() {
        let board = parlor_chess::Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )
        .expect("start position FEN should parse");
        assert_eq!(board.legal_moves().len(), 20);
    }
}
