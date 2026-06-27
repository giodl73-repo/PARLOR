//! A chess kernel: board, FEN, fully legal move generation, and perft.
//!
//! Square indexing is 0 = a1, 1 = b1, ... 7 = h1, 8 = a2, ... 63 = h8, so
//! `rank = sq / 8` and `file = sq % 8`. Move generation is pseudo-legal followed
//! by a king-safety filter (make the move on a copy; reject if the mover's king
//! is attacked). Correctness is pinned to published perft counts in the tests.

#![forbid(unsafe_code)]

use parlor_core::{EvidenceLabel, Game, Perft, PerftBenchmark, PerftCheck};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn flip(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

/// Castling-right bit flags.
const WK: u8 = 1;
const WQ: u8 = 2;
const BK: u8 = 4;
const BQ: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveFlag {
    Normal,
    DoublePush,
    EnPassant,
    CastleKing,
    CastleQueen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<PieceKind>,
    pub flag: MoveFlag,
}

/// A chess position. `Copy` so move generation can use cheap copy-make.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    squares: [Option<Piece>; 64],
    side: Color,
    castling: u8,
    ep: Option<u8>,
}

#[inline]
fn rank_of(sq: u8) -> u8 {
    sq / 8
}
#[inline]
fn file_of(sq: u8) -> u8 {
    sq % 8
}
#[inline]
fn sq_of(file: i32, rank: i32) -> Option<u8> {
    if (0..8).contains(&file) && (0..8).contains(&rank) {
        Some((rank * 8 + file) as u8)
    } else {
        None
    }
}

const KNIGHT_DELTAS: [(i32, i32); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];
const KING_DELTAS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const BISHOP_DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ROOK_DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Debug, PartialEq, Eq)]
pub enum FenError {
    Fields,
    Board,
    Color,
    Square,
}

impl Board {
    /// The standard chess starting position.
    pub fn start() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("startpos FEN is valid")
    }

    pub fn side_to_move(&self) -> Color {
        self.side
    }

    pub fn piece_at(&self, sq: u8) -> Option<Piece> {
        self.squares[sq as usize]
    }

    /// Parse a position from Forsyth-Edwards Notation. Halfmove/fullmove fields
    /// are accepted but ignored (they do not affect move generation or perft).
    pub fn from_fen(fen: &str) -> Result<Board, FenError> {
        let fields: Vec<&str> = fen.split_whitespace().collect();
        if fields.len() < 4 {
            return Err(FenError::Fields);
        }
        let mut squares = [None; 64];
        let ranks: Vec<&str> = fields[0].split('/').collect();
        if ranks.len() != 8 {
            return Err(FenError::Board);
        }
        // FEN lists rank 8 first; our index 0 is a1 (rank 1).
        for (i, row) in ranks.iter().enumerate() {
            let rank = 7 - i as i32;
            let mut file = 0i32;
            for ch in row.chars() {
                if let Some(skip) = ch.to_digit(10) {
                    file += skip as i32;
                    continue;
                }
                let color = if ch.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let kind = match ch.to_ascii_lowercase() {
                    'p' => PieceKind::Pawn,
                    'n' => PieceKind::Knight,
                    'b' => PieceKind::Bishop,
                    'r' => PieceKind::Rook,
                    'q' => PieceKind::Queen,
                    'k' => PieceKind::King,
                    _ => return Err(FenError::Board),
                };
                let sq = sq_of(file, rank).ok_or(FenError::Board)?;
                squares[sq as usize] = Some(Piece { color, kind });
                file += 1;
            }
            if file != 8 {
                return Err(FenError::Board);
            }
        }
        let side = match fields[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::Color),
        };
        let mut castling = 0u8;
        if fields[2] != "-" {
            for ch in fields[2].chars() {
                match ch {
                    'K' => castling |= WK,
                    'Q' => castling |= WQ,
                    'k' => castling |= BK,
                    'q' => castling |= BQ,
                    _ => return Err(FenError::Board),
                }
            }
        }
        let ep = if fields[3] == "-" {
            None
        } else {
            Some(parse_square(fields[3]).ok_or(FenError::Square)?)
        };
        Ok(Board {
            squares,
            side,
            castling,
            ep,
        })
    }

    fn king_square(&self, color: Color) -> Option<u8> {
        (0..64u8).find(|&sq| {
            self.squares[sq as usize]
                == Some(Piece {
                    color,
                    kind: PieceKind::King,
                })
        })
    }

    /// Is `sq` attacked by any piece of `by`?
    pub fn is_attacked(&self, sq: u8, by: Color) -> bool {
        let f = file_of(sq) as i32;
        let r = rank_of(sq) as i32;

        // Pawn attacks: a `by`-pawn attacks diagonally forward, so it sits on the
        // square diagonally *behind* `sq` from `by`'s perspective.
        let pawn_rank = match by {
            Color::White => r - 1,
            Color::Black => r + 1,
        };
        for df in [-1, 1] {
            if let Some(p) = sq_of(f + df, pawn_rank) {
                if self.squares[p as usize]
                    == Some(Piece {
                        color: by,
                        kind: PieceKind::Pawn,
                    })
                {
                    return true;
                }
            }
        }
        // Knights.
        for (df, dr) in KNIGHT_DELTAS {
            if let Some(p) = sq_of(f + df, r + dr) {
                if self.squares[p as usize]
                    == Some(Piece {
                        color: by,
                        kind: PieceKind::Knight,
                    })
                {
                    return true;
                }
            }
        }
        // King.
        for (df, dr) in KING_DELTAS {
            if let Some(p) = sq_of(f + df, r + dr) {
                if self.squares[p as usize]
                    == Some(Piece {
                        color: by,
                        kind: PieceKind::King,
                    })
                {
                    return true;
                }
            }
        }
        // Sliding: bishops/queens on diagonals, rooks/queens on orthogonals.
        if self.slider_hits(f, r, &BISHOP_DIRS, by, PieceKind::Bishop) {
            return true;
        }
        if self.slider_hits(f, r, &ROOK_DIRS, by, PieceKind::Rook) {
            return true;
        }
        false
    }

    fn slider_hits(
        &self,
        f: i32,
        r: i32,
        dirs: &[(i32, i32)],
        by: Color,
        straight_kind: PieceKind,
    ) -> bool {
        for (df, dr) in dirs {
            let (mut cf, mut cr) = (f + df, r + dr);
            while let Some(p) = sq_of(cf, cr) {
                if let Some(piece) = self.squares[p as usize] {
                    if piece.color == by
                        && (piece.kind == straight_kind || piece.kind == PieceKind::Queen)
                    {
                        return true;
                    }
                    break;
                }
                cf += df;
                cr += dr;
            }
        }
        false
    }

    fn in_check(&self, color: Color) -> bool {
        match self.king_square(color) {
            Some(k) => self.is_attacked(k, color.flip()),
            None => false,
        }
    }

    /// All fully legal moves for the side to move.
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut pseudo = Vec::with_capacity(48);
        self.generate_pseudo(&mut pseudo);
        let mover = self.side;
        pseudo
            .into_iter()
            .filter(|mv| {
                let next = self.make(mv);
                !next.in_check(mover)
            })
            .collect()
    }

    fn generate_pseudo(&self, out: &mut Vec<Move>) {
        for sq in 0..64u8 {
            let Some(piece) = self.squares[sq as usize] else {
                continue;
            };
            if piece.color != self.side {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => self.gen_pawn(sq, out),
                PieceKind::Knight => self.gen_steps(sq, &KNIGHT_DELTAS, out),
                PieceKind::King => {
                    self.gen_steps(sq, &KING_DELTAS, out);
                    self.gen_castles(sq, out);
                }
                PieceKind::Bishop => self.gen_slides(sq, &BISHOP_DIRS, out),
                PieceKind::Rook => self.gen_slides(sq, &ROOK_DIRS, out),
                PieceKind::Queen => {
                    self.gen_slides(sq, &BISHOP_DIRS, out);
                    self.gen_slides(sq, &ROOK_DIRS, out);
                }
            }
        }
    }

    fn gen_steps(&self, from: u8, deltas: &[(i32, i32)], out: &mut Vec<Move>) {
        let f = file_of(from) as i32;
        let r = rank_of(from) as i32;
        for (df, dr) in deltas {
            if let Some(to) = sq_of(f + df, r + dr) {
                match self.squares[to as usize] {
                    Some(p) if p.color == self.side => {}
                    _ => out.push(Move {
                        from,
                        to,
                        promotion: None,
                        flag: MoveFlag::Normal,
                    }),
                }
            }
        }
    }

    fn gen_slides(&self, from: u8, dirs: &[(i32, i32)], out: &mut Vec<Move>) {
        let f = file_of(from) as i32;
        let r = rank_of(from) as i32;
        for (df, dr) in dirs {
            let (mut cf, mut cr) = (f + df, r + dr);
            while let Some(to) = sq_of(cf, cr) {
                match self.squares[to as usize] {
                    Some(p) if p.color == self.side => break,
                    Some(_) => {
                        out.push(Move {
                            from,
                            to,
                            promotion: None,
                            flag: MoveFlag::Normal,
                        });
                        break;
                    }
                    None => out.push(Move {
                        from,
                        to,
                        promotion: None,
                        flag: MoveFlag::Normal,
                    }),
                }
                cf += df;
                cr += dr;
            }
        }
    }

    fn gen_pawn(&self, from: u8, out: &mut Vec<Move>) {
        let f = file_of(from) as i32;
        let r = rank_of(from) as i32;
        let (dir, start_rank, promo_rank) = match self.side {
            Color::White => (1, 1, 7),
            Color::Black => (-1, 6, 0),
        };
        // Single push.
        if let Some(one) = sq_of(f, r + dir) {
            if self.squares[one as usize].is_none() {
                self.push_pawn(from, one, r + dir == promo_rank, MoveFlag::Normal, out);
                // Double push.
                if r == start_rank {
                    if let Some(two) = sq_of(f, r + 2 * dir) {
                        if self.squares[two as usize].is_none() {
                            out.push(Move {
                                from,
                                to: two,
                                promotion: None,
                                flag: MoveFlag::DoublePush,
                            });
                        }
                    }
                }
            }
        }
        // Captures.
        for df in [-1, 1] {
            if let Some(to) = sq_of(f + df, r + dir) {
                if let Some(p) = self.squares[to as usize] {
                    if p.color != self.side {
                        self.push_pawn(from, to, r + dir == promo_rank, MoveFlag::Normal, out);
                    }
                } else if Some(to) == self.ep {
                    out.push(Move {
                        from,
                        to,
                        promotion: None,
                        flag: MoveFlag::EnPassant,
                    });
                }
            }
        }
    }

    fn push_pawn(&self, from: u8, to: u8, promote: bool, flag: MoveFlag, out: &mut Vec<Move>) {
        if promote {
            for kind in [
                PieceKind::Queen,
                PieceKind::Rook,
                PieceKind::Bishop,
                PieceKind::Knight,
            ] {
                out.push(Move {
                    from,
                    to,
                    promotion: Some(kind),
                    flag,
                });
            }
        } else {
            out.push(Move {
                from,
                to,
                promotion: None,
                flag,
            });
        }
    }

    fn gen_castles(&self, from: u8, out: &mut Vec<Move>) {
        let opp = self.side.flip();
        match self.side {
            Color::White => {
                if from != 4 {
                    return;
                }
                if self.castling & WK != 0
                    && self.squares[5].is_none()
                    && self.squares[6].is_none()
                    && !self.is_attacked(4, opp)
                    && !self.is_attacked(5, opp)
                    && !self.is_attacked(6, opp)
                {
                    out.push(Move {
                        from,
                        to: 6,
                        promotion: None,
                        flag: MoveFlag::CastleKing,
                    });
                }
                if self.castling & WQ != 0
                    && self.squares[1].is_none()
                    && self.squares[2].is_none()
                    && self.squares[3].is_none()
                    && !self.is_attacked(4, opp)
                    && !self.is_attacked(3, opp)
                    && !self.is_attacked(2, opp)
                {
                    out.push(Move {
                        from,
                        to: 2,
                        promotion: None,
                        flag: MoveFlag::CastleQueen,
                    });
                }
            }
            Color::Black => {
                if from != 60 {
                    return;
                }
                if self.castling & BK != 0
                    && self.squares[61].is_none()
                    && self.squares[62].is_none()
                    && !self.is_attacked(60, opp)
                    && !self.is_attacked(61, opp)
                    && !self.is_attacked(62, opp)
                {
                    out.push(Move {
                        from,
                        to: 62,
                        promotion: None,
                        flag: MoveFlag::CastleKing,
                    });
                }
                if self.castling & BQ != 0
                    && self.squares[57].is_none()
                    && self.squares[58].is_none()
                    && self.squares[59].is_none()
                    && !self.is_attacked(60, opp)
                    && !self.is_attacked(59, opp)
                    && !self.is_attacked(58, opp)
                {
                    out.push(Move {
                        from,
                        to: 58,
                        promotion: None,
                        flag: MoveFlag::CastleQueen,
                    });
                }
            }
        }
    }

    /// Apply `mv`, returning the resulting position. Assumes `mv` was produced by
    /// this kernel for `self` (copy-make; the original is untouched).
    pub fn make(&self, mv: &Move) -> Board {
        let mut b = *self;
        let mut piece = b.squares[mv.from as usize].expect("move from an occupied square");

        // En-passant capture removes the pawn beside the destination, not on it.
        if mv.flag == MoveFlag::EnPassant {
            let cap_rank = rank_of(mv.from);
            let cap = (cap_rank * 8 + file_of(mv.to)) as usize;
            b.squares[cap] = None;
        }

        b.squares[mv.from as usize] = None;
        if let Some(kind) = mv.promotion {
            piece.kind = kind;
        }
        b.squares[mv.to as usize] = Some(piece);

        // Move the rook when castling.
        match mv.flag {
            MoveFlag::CastleKing => {
                let (rf, rt) = if self.side == Color::White {
                    (7, 5)
                } else {
                    (63, 61)
                };
                b.squares[rt] = b.squares[rf].take();
            }
            MoveFlag::CastleQueen => {
                let (rf, rt) = if self.side == Color::White {
                    (0, 3)
                } else {
                    (56, 59)
                };
                b.squares[rt] = b.squares[rf].take();
            }
            _ => {}
        }

        // Update castling rights: king moves, rook moves, or rook captured.
        b.clear_castle_on_touch(mv.from);
        b.clear_castle_on_touch(mv.to);
        if piece.kind == PieceKind::King {
            match self.side {
                Color::White => b.castling &= !(WK | WQ),
                Color::Black => b.castling &= !(BK | BQ),
            }
        }

        // En-passant target only after a double push.
        b.ep = if mv.flag == MoveFlag::DoublePush {
            Some((mv.from + mv.to) / 2)
        } else {
            None
        };

        b.side = self.side.flip();
        b
    }

    fn clear_castle_on_touch(&mut self, sq: u8) {
        match sq {
            0 => self.castling &= !WQ,
            7 => self.castling &= !WK,
            56 => self.castling &= !BQ,
            63 => self.castling &= !BK,
            _ => {}
        }
    }

    /// Count leaf nodes of the legal-move tree to `depth` ply. `perft(0) == 1`.
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }
        moves.iter().map(|mv| self.make(mv).perft(depth - 1)).sum()
    }

    /// Per-move leaf counts ("divide"), the standard perft debugging view.
    pub fn perft_divide(&self, depth: u32) -> Vec<(Move, u64)> {
        self.legal_moves()
            .into_iter()
            .map(|mv| {
                let nodes = if depth <= 1 {
                    1
                } else {
                    self.make(&mv).perft(depth - 1)
                };
                (mv, nodes)
            })
            .collect()
    }
}

fn parse_square(s: &str) -> Option<u8> {
    let bytes = s.as_bytes();
    if bytes.len() != 2 {
        return None;
    }
    let file = bytes[0].checked_sub(b'a')? as i32;
    let rank = bytes[1].checked_sub(b'1')? as i32;
    sq_of(file, rank)
}

/// Render a square index as algebraic coordinates, e.g. 0 -> "a1".
pub fn square_name(sq: u8) -> String {
    let file = (b'a' + file_of(sq)) as char;
    let rank = (b'1' + rank_of(sq)) as char;
    format!("{file}{rank}")
}

/// Render a move in long algebraic notation, e.g. "e2e4", "e7e8q".
pub fn move_name(mv: &Move) -> String {
    let mut s = format!("{}{}", square_name(mv.from), square_name(mv.to));
    if let Some(kind) = mv.promotion {
        s.push(match kind {
            PieceKind::Queen => 'q',
            PieceKind::Rook => 'r',
            PieceKind::Bishop => 'b',
            PieceKind::Knight => 'n',
            _ => '?',
        });
    }
    s
}

/// The chess game handle implementing the cross-parlor traits.
#[derive(Clone, Copy, Debug, Default)]
pub struct Chess;

impl Game for Chess {
    type Position = Board;
    fn id(&self) -> &'static str {
        "chess"
    }
    fn name(&self) -> &'static str {
        "Chess"
    }
    fn initial_position(&self) -> Board {
        Board::start()
    }
}

impl Perft for Chess {
    fn perft(&self, pos: &Board, depth: u32) -> u64 {
        pos.perft(depth)
    }
}

/// Published perft benchmarks used to verify the move generator.
///
/// Source: chessprogramming wiki, "Perft Results"
/// (<https://www.chessprogramming.org/Perft_Results>). Startpos counts and the
/// "Kiwipete" position (which exercises castling, en passant, and promotions)
/// are canonical and exact.
pub const BENCHMARKS: &[PerftBenchmark] = &[
    PerftBenchmark {
        name: "startpos-d1",
        position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        depth: 1,
        expected_nodes: 20,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "startpos-d2",
        position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        depth: 2,
        expected_nodes: 400,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "startpos-d3",
        position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        depth: 3,
        expected_nodes: 8902,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "startpos-d4",
        position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        depth: 4,
        expected_nodes: 197_281,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "kiwipete-d1",
        position: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        depth: 1,
        expected_nodes: 48,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "kiwipete-d2",
        position: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        depth: 2,
        expected_nodes: 2039,
        source: "chessprogramming wiki, Perft Results",
    },
    PerftBenchmark {
        name: "kiwipete-d3",
        position: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        depth: 3,
        expected_nodes: 97_862,
        source: "chessprogramming wiki, Perft Results",
    },
];

/// Run a single benchmark against the kernel. The result is labelled
/// [`EvidenceLabel::Proven`] because a perft count is established by exhaustive
/// enumeration of the legal-move tree.
pub fn check(benchmark: &PerftBenchmark) -> PerftCheck {
    let board = Board::from_fen(benchmark.position).expect("benchmark FEN is valid");
    PerftCheck {
        benchmark: benchmark.clone(),
        observed_nodes: board.perft(benchmark.depth),
        label: EvidenceLabel::Proven,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    #[test]
    fn perft_zero_is_one() {
        assert_eq!(Board::start().perft(0), 1);
    }

    // Exact published values (chessprogramming wiki, "Perft Results"). Any
    // mismatch is a move-generation defect — the expected values are ground truth.
    #[test]
    fn startpos_has_twenty_legal_moves() {
        assert_eq!(Board::start().legal_moves().len(), 20);
    }

    #[test]
    fn startpos_perft_two_is_400() {
        assert_eq!(Board::start().perft(2), 400);
    }

    #[test]
    fn startpos_perft_three_is_8902() {
        assert_eq!(Board::start().perft(3), 8902);
    }

    #[test]
    fn kiwipete_perft_one_is_48() {
        let board = Board::from_fen(KIWIPETE).expect("valid FEN");
        assert_eq!(board.perft(1), 48);
    }

    #[test]
    fn kiwipete_perft_two_is_2039() {
        let board = Board::from_fen(KIWIPETE).expect("valid FEN");
        assert_eq!(board.perft(2), 2039);
    }

    #[test]
    fn move_name_renders_long_algebraic() {
        let mv = Move {
            from: 12,
            to: 28,
            promotion: None,
            flag: MoveFlag::DoublePush,
        };
        assert_eq!(move_name(&mv), "e2e4");
        let promo = Move {
            from: 52,
            to: 60,
            promotion: Some(PieceKind::Queen),
            flag: MoveFlag::Normal,
        };
        assert_eq!(move_name(&promo), "e7e8q");
    }

    #[test]
    fn published_benchmarks_pass_and_are_proven() {
        for b in BENCHMARKS {
            let result = check(b);
            assert!(result.label.is_proven());
            assert!(
                result.passed(),
                "{}: expected {}, observed {}",
                b.name,
                b.expected_nodes,
                result.observed_nodes
            );
        }
    }

    #[test]
    fn chess_handle_reports_identity_and_start() {
        let chess = Chess;
        assert_eq!(chess.id(), "chess");
        assert_eq!(chess.name(), "Chess");
        assert_eq!(Perft::perft(&chess, &chess.initial_position(), 3), 8902);
    }
}
