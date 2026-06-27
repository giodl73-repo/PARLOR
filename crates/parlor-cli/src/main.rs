use std::env;
use std::process::ExitCode;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_usage();
        return ExitCode::SUCCESS;
    }

    match args[0].as_str() {
        "games" => {
            print_games();
            ExitCode::SUCCESS
        }
        "chess" => chess_command(&args[1..]),
        "backgammon" => backgammon_command(&args[1..]),
        other => {
            eprintln!("unknown command: {}", other);
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    println!("parlor-cli: the front door for a parlor of games");
    println!();
    println!("USAGE:");
    println!("  parlor-cli games");
    println!("  parlor-cli chess verify");
    println!("  parlor-cli chess perft [--fen <FEN>] [--depth N] [--divide]");
    println!("  parlor-cli chess moves [--fen <FEN>]");
    println!("  parlor-cli backgammon pip");
    println!("  parlor-cli backgammon verify");
    println!("  parlor-cli backgammon roll <d1> <d2>");
}

fn print_games() {
    println!("{:<14}{:<14}STATUS", "GAME", "ID");
    println!("{:<14}{:<14}implemented", "Chess", "chess");
    println!("{:<14}{:<14}implemented", "Backgammon", "backgammon");
    println!("{:<14}{:<14}planned", "Checkers", "checkers");
    println!("{:<14}{:<14}planned", "Go", "go");
}

fn chess_command(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("usage: chess <verify|perft|moves> ...");
        return ExitCode::FAILURE;
    }

    match args[0].as_str() {
        "verify" => chess_verify(),
        "perft" => chess_perft(&args[1..]),
        "moves" => chess_moves(&args[1..]),
        other => {
            eprintln!("unknown chess subcommand: {}", other);
            ExitCode::FAILURE
        }
    }
}

fn chess_verify() -> ExitCode {
    let mut all_passed = true;

    for benchmark in parlor_chess::BENCHMARKS {
        let check = parlor_chess::check(benchmark);
        let passed = check.passed();
        if !passed {
            all_passed = false;
        }
        let status = if passed { "[PASS]" } else { "[FAIL]" };
        println!(
            "{} {} depth {} expected {} observed {}",
            status,
            check.benchmark.name,
            check.benchmark.depth,
            check.benchmark.expected_nodes,
            check.observed_nodes,
        );
    }

    if all_passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn chess_perft(args: &[String]) -> ExitCode {
    let mut fen = DEFAULT_FEN.to_string();
    let mut depth: u32 = 1;
    let mut divide = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fen" => {
                if i + 1 >= args.len() {
                    eprintln!("--fen requires an argument");
                    return ExitCode::FAILURE;
                }
                fen = args[i + 1].clone();
                i += 2;
            }
            "--depth" => {
                if i + 1 >= args.len() {
                    eprintln!("--depth requires an argument");
                    return ExitCode::FAILURE;
                }
                match args[i + 1].parse::<u32>() {
                    Ok(d) => depth = d,
                    Err(_) => {
                        eprintln!("invalid depth: {}", args[i + 1]);
                        return ExitCode::FAILURE;
                    }
                }
                i += 2;
            }
            "--divide" => {
                divide = true;
                i += 1;
            }
            other => {
                eprintln!("unexpected argument: {}", other);
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN ({:?}): {}", e, fen);
            return ExitCode::FAILURE;
        }
    };

    if divide {
        let mut total: u64 = 0;
        for (mv, count) in board.perft_divide(depth) {
            println!("{}: {}", parlor_chess::move_name(&mv), count);
            total += count;
        }
        println!("total: {}", total);
    } else {
        println!("{}", board.perft(depth));
    }

    ExitCode::SUCCESS
}

fn chess_moves(args: &[String]) -> ExitCode {
    let mut fen = DEFAULT_FEN.to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fen" => {
                if i + 1 >= args.len() {
                    eprintln!("--fen requires an argument");
                    return ExitCode::FAILURE;
                }
                fen = args[i + 1].clone();
                i += 2;
            }
            other => {
                eprintln!("unexpected argument: {}", other);
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN ({:?}): {}", e, fen);
            return ExitCode::FAILURE;
        }
    };

    let moves = board.legal_moves();
    println!("{} legal moves:", moves.len());
    for mv in &moves {
        println!("{}", parlor_chess::move_name(mv));
    }

    ExitCode::SUCCESS
}

fn backgammon_command(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("usage: backgammon <pip|verify|roll> ...");
        return ExitCode::FAILURE;
    }

    match args[0].as_str() {
        "pip" => backgammon_pip(),
        "verify" => backgammon_verify(),
        "roll" => backgammon_roll(&args[1..]),
        other => {
            eprintln!("unknown backgammon subcommand: {}", other);
            ExitCode::FAILURE
        }
    }
}

fn backgammon_pip() -> ExitCode {
    let board = parlor_backgammon::Board::start();
    println!(
        "White: {}",
        board.pip_count(parlor_backgammon::Player::White)
    );
    println!(
        "Black: {}",
        board.pip_count(parlor_backgammon::Player::Black)
    );
    ExitCode::SUCCESS
}

fn backgammon_verify() -> ExitCode {
    let mut all_passed = true;

    let board = parlor_backgammon::Board::start();
    let white_pips = board.pip_count(parlor_backgammon::Player::White);
    let black_pips = board.pip_count(parlor_backgammon::Player::Black);
    let pip_ok = white_pips == 167 && black_pips == 167;
    if !pip_ok {
        all_passed = false;
    }
    println!(
        "{} opening pip count is 167 per player (white {}, black {})",
        if pip_ok { "[PASS]" } else { "[FAIL]" },
        white_pips,
        black_pips,
    );

    let rolls = parlor_backgammon::Dice::distinct_rolls();
    let prob_sum: f64 = rolls.iter().map(|r| r.probability).sum();
    let prob_ok = (prob_sum - 1.0).abs() < 1e-9;
    if !prob_ok {
        all_passed = false;
    }
    println!(
        "{} the {} dice rolls' probabilities sum to 1 (sum {})",
        if prob_ok { "[PASS]" } else { "[FAIL]" },
        rolls.len(),
        prob_sum,
    );

    let mean = parlor_backgammon::Dice::expected_pips();
    let expected_mean = 49.0_f64 / 6.0;
    let mean_ok = (mean - expected_mean).abs() < 1e-9;
    if !mean_ok {
        all_passed = false;
    }
    println!(
        "{} mean roll value is 49/6 (value {})",
        if mean_ok { "[PASS]" } else { "[FAIL]" },
        mean,
    );

    if all_passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn backgammon_roll(args: &[String]) -> ExitCode {
    if args.len() < 2 {
        eprintln!("usage: backgammon roll <d1> <d2>");
        return ExitCode::FAILURE;
    }

    let die_a: u8 = match args[0].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("invalid die value: {}", args[0]);
            return ExitCode::FAILURE;
        }
    };
    let die_b: u8 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("invalid die value: {}", args[1]);
            return ExitCode::FAILURE;
        }
    };

    let plays = parlor_backgammon::Board::start().legal_plays(
        parlor_backgammon::Player::White,
        die_a,
        die_b,
    );
    println!("{}", plays.len());

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn chess_opening_has_twenty_moves() {
        assert_eq!(parlor_chess::Board::start().legal_moves().len(), 20);
    }

    #[test]
    fn backgammon_opening_pip_count_is_167() {
        let board = parlor_backgammon::Board::start();
        assert_eq!(board.pip_count(parlor_backgammon::Player::White), 167);
        assert_eq!(board.pip_count(parlor_backgammon::Player::Black), 167);
    }
}
