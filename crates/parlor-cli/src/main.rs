use std::env;
use std::process::ExitCode;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("games") => {
            cmd_games();
            ExitCode::SUCCESS
        }
        Some("chess") => cmd_chess(&args[1..]),
        Some("backgammon") => cmd_backgammon(&args[1..]),
        Some("checkers") => cmd_checkers(&args[1..]),
        Some("go") => cmd_go(&args[1..]),
        Some(other) => {
            eprintln!("unknown game: {other}");
            ExitCode::FAILURE
        }
        None => {
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    eprintln!("usage: parlor-cli <games|chess|backgammon|checkers|go> ...");
}

fn tag(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "FAIL"
    }
}

fn cmd_games() {
    println!("{:<12} {:<11} STATUS", "GAME", "ID");
    println!("{:<12} {:<11} ------", "----", "--");
    for (name, id) in [
        ("Chess", "chess"),
        ("Backgammon", "backgammon"),
        ("Checkers", "checkers"),
        ("Go", "go"),
    ] {
        println!("{:<12} {:<11} implemented", name, id);
    }
}

// ---------------------------------------------------------------------------
// Chess
// ---------------------------------------------------------------------------

fn cmd_chess(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => chess_verify(),
        Some("perft") => chess_perft(&args[1..]),
        Some("moves") => chess_moves(&args[1..]),
        _ => {
            eprintln!("usage: chess <verify|perft|moves>");
            ExitCode::FAILURE
        }
    }
}

fn chess_verify() -> ExitCode {
    let mut ok = true;
    for benchmark in parlor_chess::BENCHMARKS {
        let check = parlor_chess::check(benchmark);
        let pass = check.passed();
        if !pass {
            ok = false;
        }
        println!(
            "[{}] {} depth {} expected {} observed {}",
            tag(pass),
            check.benchmark.name,
            check.benchmark.depth,
            check.benchmark.expected_nodes,
            check.observed_nodes
        );
    }
    if ok {
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
                    match args[i + 1].parse() {
                        Ok(d) => depth = d,
                        Err(_) => {
                            eprintln!("invalid depth: {}", args[i + 1]);
                            return ExitCode::FAILURE;
                        }
                    }
                    i += 2;
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
                eprintln!("unknown option: {other}");
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
        let mut total: u64 = 0;
        for m in board.legal_moves() {
            let count = if depth >= 1 {
                board.make(&m).perft(depth - 1)
            } else {
                0
            };
            println!("{}: {}", parlor_chess::move_name(&m), count);
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
                if i + 1 < args.len() {
                    fen = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("--fen requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown option: {other}");
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
    let moves = board.legal_moves();
    for m in &moves {
        println!("{}", parlor_chess::move_name(m));
    }
    println!("{} legal moves", moves.len());
    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Backgammon
// ---------------------------------------------------------------------------

fn cmd_backgammon(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("pip") => backgammon_pip(),
        Some("verify") => backgammon_verify(),
        Some("roll") => backgammon_roll(&args[1..]),
        _ => {
            eprintln!("usage: backgammon <pip|verify|roll>");
            ExitCode::FAILURE
        }
    }
}

fn backgammon_pip() -> ExitCode {
    let board = parlor_backgammon::Board::start();
    for player in [
        parlor_backgammon::Player::Black,
        parlor_backgammon::Player::White,
    ] {
        println!("{:?}: {}", player, board.pip_count(player));
    }
    ExitCode::SUCCESS
}

fn opening_rolls() -> Vec<(u8, u8, f64, u32)> {
    let mut rolls = Vec::new();
    for d1 in 1..=6u8 {
        for d2 in d1..=6u8 {
            let probability = if d1 == d2 { 1.0 / 36.0 } else { 2.0 / 36.0 };
            let pips = if d1 == d2 {
                4 * d1 as u32
            } else {
                d1 as u32 + d2 as u32
            };
            rolls.push((d1, d2, probability, pips));
        }
    }
    rolls
}

fn backgammon_verify() -> ExitCode {
    let mut ok = true;
    let board = parlor_backgammon::Board::start();

    for player in [
        parlor_backgammon::Player::Black,
        parlor_backgammon::Player::White,
    ] {
        let pip = board.pip_count(player);
        let pass = pip == 167;
        if !pass {
            ok = false;
        }
        println!(
            "[{}] opening pip {:?} = {} (expected 167)",
            tag(pass),
            player,
            pip
        );
    }

    let rolls = opening_rolls();

    let count_ok = rolls.len() == 21;
    if !count_ok {
        ok = false;
    }
    println!(
        "[{}] distinct dice rolls = {} (expected 21)",
        tag(count_ok),
        rolls.len()
    );

    let sum: f64 = rolls.iter().map(|r| r.2).sum();
    let sum_ok = (sum - 1.0).abs() < 1e-9;
    if !sum_ok {
        ok = false;
    }
    println!(
        "[{}] dice roll probabilities sum = {} (expected 1)",
        tag(sum_ok),
        sum
    );

    let mean: f64 = rolls.iter().map(|r| r.2 * r.3 as f64).sum();
    let expected_mean = 49.0 / 6.0;
    let mean_ok = (mean - expected_mean).abs() < 1e-9;
    if !mean_ok {
        ok = false;
    }
    println!(
        "[{}] mean roll = {} (expected {})",
        tag(mean_ok),
        mean,
        expected_mean
    );

    if ok {
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
    let board = parlor_backgammon::Board::start();
    let plays = board.legal_plays(parlor_backgammon::Player::White, d1, d2);
    println!("{} distinct legal plays for {}-{}", plays.len(), d1, d2);
    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Checkers
// ---------------------------------------------------------------------------

fn cmd_checkers(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => checkers_verify(),
        Some("perft") => checkers_perft(&args[1..]),
        Some("moves") => {
            println!("{}", parlor_checkers::Board::start().legal_moves().len());
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: checkers <verify|perft|moves>");
            ExitCode::FAILURE
        }
    }
}

fn checkers_verify() -> ExitCode {
    let mut ok = true;
    for benchmark in parlor_checkers::BENCHMARKS {
        let check = parlor_checkers::check(benchmark);
        let pass = check.passed();
        if !pass {
            ok = false;
        }
        println!(
            "[{}] {} depth {} expected {} observed {}",
            tag(pass),
            check.benchmark.name,
            check.benchmark.depth,
            check.benchmark.expected_nodes,
            check.observed_nodes
        );
    }
    if ok {
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
                    match args[i + 1].parse() {
                        Ok(d) => depth = d,
                        Err(_) => {
                            eprintln!("invalid depth: {}", args[i + 1]);
                            return ExitCode::FAILURE;
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("--depth requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::FAILURE;
            }
        }
    }
    println!("{}", parlor_checkers::Board::start().perft(depth));
    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Go
// ---------------------------------------------------------------------------

fn cmd_go(args: &[String]) -> ExitCode {
    match args.first().map(|s| s.as_str()) {
        Some("verify") => go_verify(),
        Some("points") => go_points(&args[1..]),
        _ => {
            eprintln!("usage: go <verify|points>");
            ExitCode::FAILURE
        }
    }
}

fn go_verify() -> ExitCode {
    let mut ok = true;

    let points_19 = parlor_go::Board::new(19).point_count();
    let pass_19 = points_19 == 361;
    if !pass_19 {
        ok = false;
    }
    println!(
        "[{}] 19x19 point count = {} (expected 361)",
        tag(pass_19),
        points_19
    );

    let points_9 = parlor_go::Board::new(9).point_count();
    let pass_9 = points_9 == 81;
    if !pass_9 {
        ok = false;
    }
    println!(
        "[{}] 9x9 point count = {} (expected 81)",
        tag(pass_9),
        points_9
    );

    let moves_9 = parlor_go::Board::new(9)
        .legal_moves(parlor_go::Color::Black)
        .len();
    let pass_moves = moves_9 == 81;
    if !pass_moves {
        ok = false;
    }
    println!(
        "[{}] 9x9 black legal moves = {} (expected 81)",
        tag(pass_moves),
        moves_9
    );

    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn go_points(args: &[String]) -> ExitCode {
    let mut size: usize = 19;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--size" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse() {
                        Ok(s) => size = s,
                        Err(_) => {
                            eprintln!("invalid size: {}", args[i + 1]);
                            return ExitCode::FAILURE;
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("--size requires a value");
                    return ExitCode::FAILURE;
                }
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::FAILURE;
            }
        }
    }
    println!("{}", parlor_go::Board::new(size).point_count());
    ExitCode::SUCCESS
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

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
    }

    #[test]
    fn checkers_opening_move_count() {
        assert_eq!(parlor_checkers::Board::start().legal_moves().len(), 7);
    }

    #[test]
    fn go_board_point_count() {
        assert_eq!(parlor_go::Board::new(19).point_count(), 361);
    }
}
