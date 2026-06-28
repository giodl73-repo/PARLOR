#![forbid(unsafe_code)]

//! English draughts (American checkers, 8x8) move-generation kernel for the
//! `parlor-checkers` crate.
//!
//! Rules implemented: 12 men per side on the dark squares of the first three
//! rows; men move/capture one square diagonally forward only and promote to a
//! king on reaching the far back rank; kings move/capture one square diagonally
//! in any of the four directions (no flying kings); capturing is mandatory; a
//! capturing piece must continue jumping while further jumps are available
//! (multi-jump); a man that reaches the back rank by a capture stops and
//! promotes (it does not continue jumping as a king that turn).

use parlor_core::{EvidenceLabel, Game, Perft, PerftBenchmark, PerftCheck};

/// The two sides. Black moves first (toward increasing rows); White moves
/// toward decreasing rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Black,
    White,
}

impl Side {
    fn opponent(self) -> Side {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    side: Side,
    king: bool,
}

/// A single legal move.
///
/// `path` lists the squares the moving piece occupies, starting with the
/// origin square and ending with the final destination. For a quiet (non
/// capturing) move `path` has length 2 and `captures` is empty. For a capture
/// `captures` lists every jumped (and removed) square in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub path: Vec<usize>,
    pub captures: Vec<usize>,
}

/// An 8x8 draughts board plus the side to move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    squares: [Option<Piece>; 64],
    to_move: Side,
}

#[inline]
fn rc(sq: usize) -> (i32, i32) {
    ((sq / 8) as i32, (sq % 8) as i32)
}

#[inline]
fn in_bounds(r: i32, c: i32) -> bool {
    (0..8).contains(&r) && (0..8).contains(&c)
}

#[inline]
fn sq_of(r: i32, c: i32) -> usize {
    (r * 8 + c) as usize
}

#[inline]
fn is_back_rank(side: Side, r: i32) -> bool {
    match side {
        Side::Black => r == 7,
        Side::White => r == 0,
    }
}

fn dirs(p: Piece) -> Vec<(i32, i32)> {
    let mut v = Vec::with_capacity(4);
    if p.king || p.side == Side::Black {
        v.push((1, -1));
        v.push((1, 1));
    }
    if p.king || p.side == Side::White {
        v.push((-1, -1));
        v.push((-1, 1));
    }
    v
}

impl Board {
    /// Standard opening setup with Black to move.
    pub fn start() -> Board {
        let mut squares = [None; 64];
        for r in 0..3usize {
            for c in 0..8usize {
                if (r + c) % 2 == 1 {
                    squares[r * 8 + c] = Some(Piece {
                        side: Side::Black,
                        king: false,
                    });
                }
            }
        }
        for r in 5..8usize {
            for c in 0..8usize {
                if (r + c) % 2 == 1 {
                    squares[r * 8 + c] = Some(Piece {
                        side: Side::White,
                        king: false,
                    });
                }
            }
        }
        Board {
            squares,
            to_move: Side::Black,
        }
    }

    /// Every legal move for the side to move, honoring the mandatory-capture
    /// rule (if any capture exists, only capture sequences are returned).
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut captures: Vec<Move> = Vec::new();

        for sq in 0..64usize {
            if let Some(p) = self.squares[sq] {
                if p.side == self.to_move {
                    let mut path = vec![sq];
                    let mut caps: Vec<usize> = Vec::new();
                    self.gen_captures(sq, p, &mut path, &mut caps, &mut captures);
                }
            }
        }

        if !captures.is_empty() {
            return captures;
        }

        let mut quiets: Vec<Move> = Vec::new();
        for sq in 0..64usize {
            if let Some(p) = self.squares[sq] {
                if p.side == self.to_move {
                    let (r, c) = rc(sq);
                    for (dr, dc) in dirs(p) {
                        let nr = r + dr;
                        let nc = c + dc;
                        if in_bounds(nr, nc) {
                            let dest = sq_of(nr, nc);
                            if self.squares[dest].is_none() {
                                quiets.push(Move {
                                    path: vec![sq, dest],
                                    captures: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }
        quiets
    }

    fn gen_captures(
        &self,
        cur: usize,
        p: Piece,
        path: &mut Vec<usize>,
        caps: &mut Vec<usize>,
        out: &mut Vec<Move>,
    ) {
        let (r, c) = rc(cur);
        let mut extended = false;

        for (dr, dc) in dirs(p) {
            let mr = r + dr;
            let mc = c + dc;
            let lr = r + 2 * dr;
            let lc = c + 2 * dc;
            if !in_bounds(lr, lc) {
                continue;
            }
            let mid = sq_of(mr, mc);
            let land = sq_of(lr, lc);

            if caps.contains(&mid) {
                continue;
            }
            match self.squares[mid] {
                Some(mp) if mp.side == p.side.opponent() => {}
                _ => continue,
            }

            // Landing must be empty. A square the moving piece has already
            // vacated (present in `path`) counts as empty; captured pieces
            // remain on the board and therefore block.
            let occupied = self.squares[land].is_some() && !path.contains(&land);
            if occupied {
                continue;
            }

            extended = true;
            path.push(land);
            caps.push(mid);

            let promotes = !p.king && is_back_rank(p.side, lr);
            if promotes {
                // A man reaching the back rank by capture stops there.
                out.push(Move {
                    path: path.clone(),
                    captures: caps.clone(),
                });
            } else {
                self.gen_captures(land, p, path, caps, out);
            }

            path.pop();
            caps.pop();
        }

        if !extended && !caps.is_empty() {
            out.push(Move {
                path: path.clone(),
                captures: caps.clone(),
            });
        }
    }

    /// Apply a move, performing all captures and any promotion, and return the
    /// resulting board with the side to move toggled.
    pub fn make(&self, m: &Move) -> Board {
        let mut b = *self;
        let from = m.path[0];
        let to = *m.path.last().unwrap();
        let mut p = b.squares[from].unwrap();

        b.squares[from] = None;
        for &cap in &m.captures {
            b.squares[cap] = None;
        }

        let (tr, _) = rc(to);
        if !p.king && is_back_rank(p.side, tr) {
            p.king = true;
        }
        b.squares[to] = Some(p);
        b.to_move = self.to_move.opponent();
        b
    }

    /// Leaf nodes of the legal-move tree. `perft(0) == 1`; `perft(1)` is the
    /// number of legal moves.
    pub fn perft(&self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }
        let mut total = 0u64;
        for m in &moves {
            total += self.make(m).perft(depth - 1);
        }
        total
    }
}

/// Game/Perft entry point for English draughts.
pub struct Checkers;

impl Game for Checkers {
    type Position = Board;

    fn id(&self) -> &'static str {
        "checkers"
    }

    fn name(&self) -> &'static str {
        "Checkers"
    }

    fn initial_position(&self) -> Self::Position {
        Board::start()
    }
}

impl Perft for Checkers {
    fn perft(&self, pos: &Self::Position, depth: u32) -> u64 {
        pos.perft(depth)
    }
}

/// Published English-draughts perft benchmarks from the opening position. The
/// `position` label is informational; checks always run from `Board::start()`.
pub const BENCHMARKS: &[PerftBenchmark] = &[
    PerftBenchmark {
        name: "startpos-perft-1",
        position: "startpos",
        depth: 1,
        expected_nodes: 7,
        source: "Published English draughts perft (opening position)",
    },
    PerftBenchmark {
        name: "startpos-perft-2",
        position: "startpos",
        depth: 2,
        expected_nodes: 49,
        source: "Published English draughts perft (opening position)",
    },
    PerftBenchmark {
        name: "startpos-perft-3",
        position: "startpos",
        depth: 3,
        expected_nodes: 302,
        source: "Published English draughts perft (opening position)",
    },
];

/// Run a benchmark from the standard opening position. The result is labelled
/// `Proven` because it is computed by exhaustive enumeration.
pub fn check(benchmark: &PerftBenchmark) -> PerftCheck {
    let observed_nodes = Board::start().perft(benchmark.depth);
    PerftCheck {
        benchmark: benchmark.clone(),
        observed_nodes,
        label: EvidenceLabel::Proven,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_perft_counts() {
        let b = Board::start();
        assert_eq!(b.perft(1), 7);
        assert_eq!(b.perft(2), 49);
        assert_eq!(b.perft(3), 302);
    }

    #[test]
    fn benchmarks_pass() {
        for bench in BENCHMARKS {
            let result = check(bench);
            assert_eq!(result.observed_nodes, bench.expected_nodes);
            assert_eq!(result.label, EvidenceLabel::Proven);
        }
    }
}
