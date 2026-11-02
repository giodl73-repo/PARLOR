//! Transport/admission only: native game crates own rules and reported quantities.
use parlor_core::Game;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
type Failure = (String, String);
fn refusal(s: impl Into<String>) -> Failure {
    ("browser_refusal".into(), s.into())
}
fn native(s: impl std::fmt::Debug) -> Failure {
    ("native_error".into(), format!("{s:?}"))
}
fn bounded(s: &str, n: usize) -> Result<(), Failure> {
    if s.len() > n {
        Err(refusal(format!("String exceeds {n} UTF-8 bytes")))
    } else {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(tag = "game", rename_all = "lowercase", deny_unknown_fields)]
enum Request {
    Chess {
        #[serde(default = "start")]
        fen: String,
        #[serde(default)]
        moves: Vec<String>,
        #[serde(default = "depth")]
        depth: u32,
    },
    Checkers {
        #[serde(default)]
        paths: Vec<Vec<u32>>,
        #[serde(default = "depth")]
        depth: u32,
    },
    Backgammon {
        #[serde(default)]
        turns: Vec<RollStep>,
        dice: [u8; 2],
        player: String,
    },
    Go {
        size: u32,
        #[serde(default)]
        placements: Vec<Placement>,
        player: String,
    },
}
fn start() -> String {
    START.into()
}
fn depth() -> u32 {
    1
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RollStep {
    player: String,
    dice: [u8; 2],
    selection: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Placement {
    row: u32,
    column: u32,
    color: String,
}
fn player(s: &str) -> Result<parlor_backgammon::Player, Failure> {
    match s {
        "white" => Ok(parlor_backgammon::Player::White),
        "black" => Ok(parlor_backgammon::Player::Black),
        _ => Err(refusal("player must be white or black")),
    }
}
fn color(s: &str) -> Result<parlor_go::Color, Failure> {
    match s {
        "white" => Ok(parlor_go::Color::White),
        "black" => Ok(parlor_go::Color::Black),
        _ => Err(refusal("color must be white or black")),
    }
}
fn dice(d: [u8; 2]) -> Result<(), Failure> {
    if d.iter().all(|x| (1..=6).contains(x)) {
        Ok(())
    } else {
        Err(refusal("dice must each be 1..6"))
    }
}
fn count_sequence(n: usize) -> Result<(), Failure> {
    if n > 128 {
        Err(refusal("At most 128 sequence entries"))
    } else {
        Ok(())
    }
}
fn visit(left: &mut usize) -> Result<(), Failure> {
    if *left == 0 {
        Err(refusal("Perft preflight exceeds 100000 visited nodes"))
    } else {
        *left -= 1;
        Ok(())
    }
}
fn chess_admit(b: &parlor_chess::Board, d: u32, left: &mut usize) -> Result<(), Failure> {
    visit(left)?;
    if d > 0 {
        for m in b.legal_moves() {
            chess_admit(&b.make(&m), d - 1, left)?;
        }
    }
    Ok(())
}
fn checkers_admit(b: &parlor_checkers::Board, d: u32, left: &mut usize) -> Result<(), Failure> {
    visit(left)?;
    if d > 0 {
        for m in b.legal_moves() {
            checkers_admit(&b.make(&m), d - 1, left)?;
        }
    }
    Ok(())
}
fn metadata(g: impl Game) -> Value {
    json!({"id":g.id(),"name":g.name(),"players":g.player_count()})
}
fn run(r: Request) -> Result<Value, Failure> {
    match r {
        Request::Chess { fen, moves, depth } => {
            bounded(&fen, 4096)?;
            count_sequence(moves.len())?;
            if depth > 3 {
                return Err(refusal("Browser perft depth is 0..3"));
            }
            let mut b = parlor_chess::Board::from_fen(&fen).map_err(native)?;
            for (i, name) in moves.iter().enumerate() {
                bounded(name, 16)?;
                let legal = b.legal_moves();
                let m = legal
                    .iter()
                    .find(|m| parlor_chess::move_name(m) == *name)
                    .ok_or_else(|| {
                        native(format!("Move {i} is not in native legal list: {name}"))
                    })?;
                b = b.make(m);
            }
            let mut left = 100000;
            chess_admit(&b, depth, &mut left)?;
            let legal = b.legal_moves();
            let divide = if depth > 0 {
                b.perft_divide(depth)
                    .into_iter()
                    .map(|(m, n)| json!({"move":parlor_chess::move_name(&m),"nodes":n.to_string()}))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };
            let pieces=(0..64u8).filter_map(|sq|b.piece_at(sq).map(|p|json!({"square":parlor_chess::square_name(sq),"color":format!("{:?}",p.color),"piece":format!("{:?}",p.kind)}))).collect::<Vec<_>>();
            let benchmarks=parlor_chess::BENCHMARKS.iter().map(|v|json!({"name":v.name,"position":v.position,"depth":v.depth,"expected_nodes":v.expected_nodes.to_string(),"source":v.source,"expected_evidence":"Repository retained reference; external citation not independently reverified","matches_request":moves.is_empty()&&fen==v.position&&depth==v.depth})).collect::<Vec<_>>();
            Ok(
                json!({"game":metadata(parlor_chess::Chess),"native_side_to_move":format!("{:?}",b.side_to_move()),"pieces":pieces,"legal_moves":legal.iter().map(parlor_chess::move_name).collect::<Vec<_>>(),"perft":{"depth":depth,"nodes":b.perft(depth).to_string(),"evidence":"Proven: native exhaustive enumeration of this kernel's move tree, not complete chess-law certification","preflight_visited":100000-left,"divide":divide},"retained_benchmarks":benchmarks,"boundary":"Native FEN acceptance is not full legal-position validation. Clock/additional fields are not reconstructed. Move application uses native legal-list membership."}),
            )
        }
        Request::Checkers { paths, depth } => {
            count_sequence(paths.len())?;
            if depth > 3 {
                return Err(refusal("Browser perft depth is 0..3"));
            }
            let mut b = parlor_checkers::Board::start();
            for (i, path) in paths.iter().enumerate() {
                if path.len() > 32 || path.iter().any(|s| *s >= 64) {
                    return Err(refusal("Checkers paths need at most32 squares in0..63"));
                }
                let legal = b.legal_moves();
                let m = legal
                    .iter()
                    .find(|m| m.path.iter().copied().eq(path.iter().map(|s| *s as usize)))
                    .ok_or_else(|| native(format!("Path {i} is not in native legal list")))?;
                b = b.make(m);
            }
            let mut left = 100000;
            checkers_admit(&b, depth, &mut left)?;
            let legal = b.legal_moves();
            let divide = if depth > 0 {
                legal.iter().map(|m|json!({"path":m.path,"captures":m.captures,"nodes":b.make(m).perft(depth-1).to_string()})).collect::<Vec<_>>()
            } else {
                vec![]
            };
            Ok(
                json!({"game":metadata(parlor_checkers::Checkers),"native_debug":format!("{b:?}"),"legal_moves":legal.iter().map(|m|json!({"path":m.path,"captures":m.captures})).collect::<Vec<_>>(),"perft":{"depth":depth,"nodes":b.perft(depth).to_string(),"preflight_visited":100000-left,"divide":divide,"evidence":"Proven: native exhaustive enumeration of this kernel's move tree"},"retained_benchmarks":parlor_checkers::BENCHMARKS.iter().map(|v|json!({"name":v.name,"depth":v.depth,"expected_nodes":v.expected_nodes.to_string(),"source":v.source,"expected_evidence":"Repository retained reference; external citation not independently reverified","matches_request":paths.is_empty()&&depth==v.depth})).collect::<Vec<_>>(),"boundary":"English draughts native kernel; row-major0..63 square indices, not conventional1..32 draughts notation. Debug text is not a portable state format."}),
            )
        }
        Request::Backgammon {
            turns,
            dice: roll,
            player: p,
        } => {
            if turns.len() > 16 {
                return Err(refusal("At most16 backgammon turn selections"));
            }
            dice(roll)?;
            let mut b = parlor_backgammon::Board::start();
            for (i, t) in turns.iter().enumerate() {
                dice(t.dice)?;
                let plays = b.legal_plays(player(&t.player)?, t.dice[0], t.dice[1]);
                b = plays
                    .get(t.selection as usize)
                    .ok_or_else(|| {
                        native(format!(
                            "Turn {i}: selection is not in native successor list"
                        ))
                    })?
                    .board
                    .clone();
            }
            let plays = b.legal_plays(player(&p)?, roll[0], roll[1]);
            Ok(
                json!({"game":metadata(parlor_backgammon::Backgammon),"native_debug":format!("{b:?}"),"pip_count":{"white":b.pip_count(parlor_backgammon::Player::White),"black":b.pip_count(parlor_backgammon::Player::Black),"evidence":"Observed native analytic pip-distance calculation"},"query_player":p,"dice":roll,"legal_plays":plays.iter().enumerate().map(|(i,p)|json!({"selection":i,"hits":p.hits,"native_debug":format!("{:?}",p.board),"white_pips":p.board.pip_count(parlor_backgammon::Player::White),"black_pips":p.board.pip_count(parlor_backgammon::Player::Black)})).collect::<Vec<_>>(),"dice_model":{"rolls":parlor_backgammon::Dice::distinct_rolls().iter().map(|d|json!({"dice":[d.die_a,d.die_b],"probability":d.probability})).collect::<Vec<_>>(),"expected_pips":parlor_backgammon::Dice::expected_pips(),"evidence":"Native analytic unordered-dice model, not sampled odds or match equity"},"boundary":"Distinct resulting boards, not exposed checker-move paths. Selection indices belong to this native enumeration; not stable IDs, best moves or turn enforcement. Debug text is not a state interchange format."}),
            )
        }
        Request::Go {
            size,
            placements,
            player: p,
        } => {
            if size == 0 || size > 19 || size % 2 == 0 {
                return Err(refusal("Browser Go size must be odd1..19"));
            }
            count_sequence(placements.len())?;
            let mut b = parlor_go::Board::new(size as usize);
            let mut events = Vec::new();
            for (i, m) in placements.iter().enumerate() {
                let captured = b
                    .play((m.row as usize, m.column as usize), color(&m.color)?)
                    .map_err(|e| native(format!("Placement {i}: {e:?}")))?;
                events.push(
                    json!({"row":m.row,"column":m.column,"color":m.color,"captured":captured}),
                );
            }
            let c = color(&p)?;
            let score = b.area_score();
            Ok(
                json!({"game":metadata(parlor_go::Go),"size":b.size(),"point_count":b.point_count(),"nominal_to_move":format!("{:?}",b.to_move()),"placements":events,"query_player":p,"legal_moves":b.legal_moves(c),"legal_move_count":b.legal_move_count(c),"area_score":{"black":score.0,"white":score.1,"evidence":"Observed native area-score computation; not adjudicated game result"},"boundary":"Explicit supplied colors: native play does not enforce alternating turns. Simple ko, suicide/capture and native area scoring only; no pass/komi/superko/adjudication. Coordinates are0-based(row,column). No invented native board getter."}),
            )
        }
    }
}
#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
    let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
    let result = if input.len() > 262144 {
        Err(refusal("Request exceeds256KiB"))
    } else {
        serde_json::from_str::<Request>(input)
            .map_err(|e| ("input_error".into(), e.to_string()))
            .and_then(run)
    };
    let mut value = match result {
        Ok(data) => json!({"kind":"report","data":data}),
        Err((kind, error)) => json!({"kind":kind,"error":error}),
    };
    value["request_sha256"] = json!(hash);
    let text = value.to_string();
    if text.len() > 2097152 {
        return json!({"kind":"browser_refusal","error":"Report exceeds2MiB","request_sha256":hash}).to_string();
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    fn eval(v: Value) -> Value {
        serde_json::from_str(&evaluate(&v.to_string())).unwrap()
    }
    #[test]
    fn native_chess_and_bad_syntax() {
        let r = eval(json!({"game":"chess","depth":3}));
        assert_eq!(r["data"]["perft"]["nodes"], "8902");
        assert_eq!(r["data"]["legal_moves"].as_array().unwrap().len(), 20);
        assert_eq!(
            eval(json!({"game":"chess","fen":"8/8"}))["kind"],
            "native_error"
        );
    }
    #[test]
    fn illegal_make_is_never_called() {
        assert_eq!(
            eval(json!({"game":"chess","moves":["e2e5"]}))["kind"],
            "native_error"
        );
        assert_eq!(
            eval(json!({"game":"checkers","paths":[[0,63]]}))["kind"],
            "native_error"
        );
    }
    #[test]
    fn native_go_accepts_explicit_same_color_and_rejects_occupied() {
        let r = eval(
            json!({"game":"go","size":5,"player":"black","placements":[{"row":0,"column":0,"color":"black"},{"row":0,"column":1,"color":"black"}]}),
        );
        assert_eq!(r["data"]["legal_move_count"], 23);
        assert_eq!(
            eval(
                json!({"game":"go","size":5,"player":"black","placements":[{"row":0,"column":0,"color":"black"},{"row":0,"column":0,"color":"white"}]})
            )["kind"],
            "native_error"
        );
    }
    #[test]
    fn native_backgammon_and_separate_admission() {
        let r = eval(json!({"game":"backgammon","dice":[1,2],"player":"white"}));
        assert_eq!(r["data"]["pip_count"]["white"], 167);
        assert!(!r["data"]["legal_plays"].as_array().unwrap().is_empty());
        assert_eq!(
            eval(json!({"game":"chess","depth":4}))["kind"],
            "browser_refusal"
        );
        assert_eq!(
            eval(json!({"game":"go","size":20,"player":"black"}))["kind"],
            "browser_refusal"
        );
    }
}
