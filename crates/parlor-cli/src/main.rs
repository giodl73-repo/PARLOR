use std::process::ExitCode;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: parlor-cli <games|chess|backgammon|checkers> ...");
        return ExitCode::FAILURE;
    }
    match args[0].as_str() {
        "games" => {
            print_games();
            ExitCode::SUCCESS
        }
        "chess" => chess_cmd(&args[1..]),
        "backgammon" => backgammon_cmd(&args[1..]),
        "checkers" => checkers_cmd(&args[1..]),
        other => {
            eprintln!("unknown command: {}", other);
            ExitCode::FAILURE
        }
    }
}

fn print_games() {
    println!("{:<12} {:<12}", "GAME", "STATUS");
    println!("{:<12} {:<12}", "chess", "implemented");
    println!("{:<12} {:<12}", "backgammon", "implemented");
    println!("{:<12} {:<12}", "checkers", "implemented");
    println!("{:<12} {:<12}", "go", "planned");
}

fn report(all_ok: &mut bool, ok: bool, label: &str) {
    if ok {
        println!("[PASS] {}", label);
    } else {
        *all_ok = false;
        println!("[FAIL] {}", label);
    }
}

fn chess_cmd(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => chess_verify(),
        Some("perft") => chess_perft(&args[1..]),
        Some("moves") => chess_moves(&args[1..]),
        _ => {
            eprintln!("usage: parlor-cli chess <verify|perft|moves>");
            ExitCode::FAILURE
        }
    }
}

fn chess_verify() -> ExitCode {
    let mut all_ok = true;
    for b in parlor_chess::BENCHMARKS {
        let c = parlor_chess::check(b);
        let status = if c.passed() {
            "PASS"
        } else {
            all_ok = false;
            "FAIL"
        };
        println!(
            "[{}] {} depth {} expected {} observed {}",
            status,
            c.benchmark.name,
            c.benchmark.depth,
            c.benchmark.expected_nodes,
            c.observed_nodes
        );
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn chess_perft(args: &[String]) -> ExitCode {
    let mut fen = START_FEN.to_string();
    let mut depth: u32 = 1;
    let mut divide = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fen" => {
                if i + 1 < args.len() {
                    fen = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--fen requires a value");
                    return ExitCode::FAILURE;
                }
            }
            "--depth" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u32>() {
                        Ok(d) => {
                            depth = d;
                            i += 2;
                        }
                        Err(_) => {
                            eprintln!("invalid depth: {}", args[i + 1]);
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
                eprintln!("unknown argument: {}", other);
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN: {:?}", e);
            return ExitCode::FAILURE;
        }
    };

    if divide {
        let parts = board.perft_divide(depth);
        let mut total: u64 = 0;
        for (mv, count) in &parts {
            println!("{}: {}", parlor_chess::move_name(mv), count);
            total += count;
        }
        println!("Total: {}", total);
    } else {
        println!("{}", board.perft(depth));
    }
    ExitCode::SUCCESS
}

fn chess_moves(args: &[String]) -> ExitCode {
    let mut fen = START_FEN.to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--fen" => {
                if i + 1 < args.len() {
                    fen = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--fen requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown argument: {}", other);
                return ExitCode::FAILURE;
            }
        }
    }

    let board = match parlor_chess::Board::from_fen(&fen) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("invalid FEN: {:?}", e);
            return ExitCode::FAILURE;
        }
    };

    for mv in board.legal_moves() {
        println!("{}", parlor_chess::move_name(&mv));
    }
    ExitCode::SUCCESS
}

fn backgammon_cmd(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("pip") => backgammon_pip(),
        Some("verify") => backgammon_verify(),
        Some("roll") => backgammon_roll(&args[1..]),
        _ => {
            eprintln!("usage: parlor-cli backgammon <pip|verify|roll>");
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
    let mut all_ok = true;
    let board = parlor_backgammon::Board::start();
    let white = board.pip_count(parlor_backgammon::Player::White);
    let black = board.pip_count(parlor_backgammon::Player::Black);
    report(
        &mut all_ok,
        white == 167 && black == 167,
        &format!(
            "opening pip count 167 per player (white {}, black {})",
            white, black
        ),
    );

    let rolls = parlor_backgammon::Dice::distinct_rolls();
    let sum: f64 = rolls.iter().map(|r| r.probability).sum();
    report(
        &mut all_ok,
        rolls.len() == 21 && (sum - 1.0).abs() < 1e-9,
        &format!(
            "21 dice rolls summing to 1 (count {}, sum {})",
            rolls.len(),
            sum
        ),
    );

    let mean: f64 = rolls
        .iter()
        .map(|r| {
            let value = if r.die_a == r.die_b {
                4u32 * (r.die_a as u32)
            } else {
                (r.die_a as u32) + (r.die_b as u32)
            };
            r.probability * (value as f64)
        })
        .sum();
    report(
        &mut all_ok,
        (mean - 49.0 / 6.0).abs() < 1e-9,
        &format!("mean roll 49/6 (observed {})", mean),
    );

    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn backgammon_roll(args: &[String]) -> ExitCode {
    if args.len() < 2 {
        eprintln!("usage: parlor-cli backgammon roll <d1> <d2>");
        return ExitCode::FAILURE;
    }
    let d1: u8 = match args[0].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("invalid die: {}", args[0]);
            return ExitCode::FAILURE;
        }
    };
    let d2: u8 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("invalid die: {}", args[1]);
            return ExitCode::FAILURE;
        }
    };

    let plays =
        parlor_backgammon::Board::start().legal_plays(parlor_backgammon::Player::White, d1, d2);
    let mut distinct: Vec<parlor_backgammon::Play> = Vec::new();
    for p in plays {
        if !distinct.contains(&p) {
            distinct.push(p);
        }
    }
    println!("{}", distinct.len());
    ExitCode::SUCCESS
}

fn checkers_cmd(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => checkers_verify(),
        Some("perft") => checkers_perft(&args[1..]),
        Some("moves") => checkers_moves(),
        _ => {
            eprintln!("usage: parlor-cli checkers <verify|perft|moves>");
            ExitCode::FAILURE
        }
    }
}

fn checkers_verify() -> ExitCode {
    let mut all_ok = true;
    for b in parlor_checkers::BENCHMARKS {
        let c = parlor_checkers::check(b);
        let status = if c.passed() {
            "PASS"
        } else {
            all_ok = false;
            "FAIL"
        };
        println!(
            "[{}] {} depth {} expected {} observed {}",
            status,
            c.benchmark.name,
            c.benchmark.depth,
            c.benchmark.expected_nodes,
            c.observed_nodes
        );
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn checkers_perft(args: &[String]) -> ExitCode {
    let mut depth: u32 = 1;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--depth" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u32>() {
                        Ok(d) => {
                            depth = d;
                            i += 2;
                        }
                        Err(_) => {
                            eprintln!("invalid depth: {}", args[i + 1]);
                            return ExitCode::FAILURE;
                        }
                    }
                } else {
                    eprintln!("--depth requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown argument: {}", other);
                return ExitCode::FAILURE;
            }
        }
    }
    println!("{}", parlor_checkers::Board::start().perft(depth));
    ExitCode::SUCCESS
}

fn checkers_moves() -> ExitCode {
    println!("{}", parlor_checkers::Board::start().legal_moves().len());
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn chess_opening_move_count() {
        assert_eq!(parlor_chess::Board::start().legal_moves().len(), 20);
    }

    #[test]
    fn backgammon_opening_pip_count() {
        let board = parlor_backgammon::Board::start();
        assert_eq!(board.pip_count(parlor_backgammon::Player::White), 167);
        assert_eq!(board.pip_count(parlor_backgammon::Player::Black), 167);
    }

    #[test]
    fn checkers_opening_move_count() {
        assert_eq!(parlor_checkers::Board::start().legal_moves().len(), 7);
    }
}
