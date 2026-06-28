#![forbid(unsafe_code)]

//! A correct Go (board game) rules and scoring kernel for the `parlor-go` crate.
//!
//! Implements standard Go: stone placement, group/liberty resolution, captures,
//! suicide prohibition (unless the move captures), and the simple (positional) ko
//! rule. Also provides area scoring and a `parlor_core::Game` implementation.

use parlor_core::Game;

/// A player color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// The opposing color.
    pub fn opponent(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

/// The state of a single point on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Point {
    Empty,
    Black,
    White,
}

impl Point {
    fn of_color(color: Color) -> Point {
        match color {
            Color::Black => Point::Black,
            Color::White => Point::White,
        }
    }
}

/// Reasons a move may be rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveError {
    /// The coordinate lies outside the board.
    OffBoard,
    /// The target point is already occupied.
    Occupied,
    /// The move would be self-capture (suicide) without capturing anything.
    Suicide,
    /// The move would recreate the position immediately prior to the
    /// opponent's last move (simple ko).
    Ko,
}

/// The result of simulating a move: the resulting grid and the captured points.
type Simulation = (Vec<Point>, Vec<(usize, usize)>);

/// A square Go board of configurable odd size.
#[derive(Clone)]
pub struct Board {
    size: usize,
    points: Vec<Point>,
    to_move: Color,
    /// Board state immediately before the last successful move, used to
    /// enforce the simple ko rule.
    prev: Option<Vec<Point>>,
}

impl Board {
    /// Create a new empty board of the given size with Black to move.
    pub fn new(size: usize) -> Board {
        Board {
            size,
            points: vec![Point::Empty; size * size],
            to_move: Color::Black,
            prev: None,
        }
    }

    /// The side length of the board.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The total number of points on the board (size * size).
    pub fn point_count(&self) -> usize {
        self.size * self.size
    }

    /// The color whose turn it is to move.
    pub fn to_move(&self) -> Color {
        self.to_move
    }

    fn index(&self, point: (usize, usize)) -> usize {
        point.0 * self.size + point.1
    }

    fn on_board(&self, point: (usize, usize)) -> bool {
        point.0 < self.size && point.1 < self.size
    }

    fn neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = point;
        let mut out = Vec::new();
        if r > 0 {
            out.push((r - 1, c));
        }
        if r + 1 < self.size {
            out.push((r + 1, c));
        }
        if c > 0 {
            out.push((r, c - 1));
        }
        if c + 1 < self.size {
            out.push((r, c + 1));
        }
        out
    }

    /// Collect the maximally-connected same-color group containing `start`
    /// within `points`, returning the group's points and its liberty count.
    fn group_and_liberties(
        &self,
        points: &[Point],
        start: (usize, usize),
    ) -> (Vec<(usize, usize)>, usize) {
        let color = points[self.index(start)];
        let mut stack = vec![start];
        let mut seen = vec![false; points.len()];
        seen[self.index(start)] = true;
        let mut group = Vec::new();
        let mut liberties = 0usize;
        let mut lib_seen = vec![false; points.len()];
        while let Some(p) = stack.pop() {
            group.push(p);
            for n in self.neighbors(p) {
                let idx = self.index(n);
                match points[idx] {
                    Point::Empty => {
                        if !lib_seen[idx] {
                            lib_seen[idx] = true;
                            liberties += 1;
                        }
                    }
                    other => {
                        if other == color && !seen[idx] {
                            seen[idx] = true;
                            stack.push(n);
                        }
                    }
                }
            }
        }
        (group, liberties)
    }

    /// Simulate placing `color` at `point`, resolving captures and checking the
    /// suicide rule. Returns the resulting grid and the captured points. Does
    /// not enforce the ko rule (handled by the caller).
    fn simulate(&self, point: (usize, usize), color: Color) -> Result<Simulation, MoveError> {
        if !self.on_board(point) {
            return Err(MoveError::OffBoard);
        }
        let idx = self.index(point);
        if self.points[idx] != Point::Empty {
            return Err(MoveError::Occupied);
        }
        let mut grid = self.points.clone();
        grid[idx] = Point::of_color(color);
        let opp = Point::of_color(color.opponent());

        let mut captured = Vec::new();
        let mut handled = vec![false; grid.len()];
        for n in self.neighbors(point) {
            let nidx = self.index(n);
            if grid[nidx] == opp && !handled[nidx] {
                let (group, libs) = self.group_and_liberties(&grid, n);
                for &g in &group {
                    handled[self.index(g)] = true;
                }
                if libs == 0 {
                    for &g in &group {
                        captured.push(g);
                    }
                }
            }
        }

        for &g in &captured {
            let gi = self.index(g);
            grid[gi] = Point::Empty;
        }

        let (_own, own_libs) = self.group_and_liberties(&grid, point);
        if own_libs == 0 && captured.is_empty() {
            return Err(MoveError::Suicide);
        }

        Ok((grid, captured))
    }

    /// Play `color` at `point`. On success returns the captured points.
    pub fn play(
        &mut self,
        point: (usize, usize),
        color: Color,
    ) -> Result<Vec<(usize, usize)>, MoveError> {
        let (grid, captured) = self.simulate(point, color)?;
        if let Some(prev) = &self.prev {
            if *prev == grid {
                return Err(MoveError::Ko);
            }
        }
        let before = self.points.clone();
        self.points = grid;
        self.prev = Some(before);
        self.to_move = color.opponent();
        Ok(captured)
    }

    /// Every empty point where `color` may legally play under the suicide and
    /// ko rules.
    pub fn legal_moves(&self, color: Color) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for r in 0..self.size {
            for c in 0..self.size {
                let point = (r, c);
                if self.points[self.index(point)] != Point::Empty {
                    continue;
                }
                if let Ok((grid, _captured)) = self.simulate(point, color) {
                    if let Some(prev) = &self.prev {
                        if *prev == grid {
                            continue;
                        }
                    }
                    moves.push(point);
                }
            }
        }
        moves
    }

    /// Area score as (black, white): stones on the board plus empty regions
    /// surrounded wholly by one color. Regions touching both colors (dame)
    /// count for neither.
    pub fn area_score(&self) -> (u32, u32) {
        let mut black = 0u32;
        let mut white = 0u32;
        let mut visited = vec![false; self.points.len()];
        for r in 0..self.size {
            for c in 0..self.size {
                let p = (r, c);
                let idx = self.index(p);
                match self.points[idx] {
                    Point::Black => black += 1,
                    Point::White => white += 1,
                    Point::Empty => {
                        if visited[idx] {
                            continue;
                        }
                        let mut stack = vec![p];
                        visited[idx] = true;
                        let mut size = 0u32;
                        let mut touch_black = false;
                        let mut touch_white = false;
                        while let Some(q) = stack.pop() {
                            size += 1;
                            for n in self.neighbors(q) {
                                let nidx = self.index(n);
                                match self.points[nidx] {
                                    Point::Empty => {
                                        if !visited[nidx] {
                                            visited[nidx] = true;
                                            stack.push(n);
                                        }
                                    }
                                    Point::Black => touch_black = true,
                                    Point::White => touch_white = true,
                                }
                            }
                        }
                        if touch_black && !touch_white {
                            black += size;
                        } else if touch_white && !touch_black {
                            white += size;
                        }
                    }
                }
            }
        }
        (black, white)
    }
}

/// The Go game definition.
pub struct Go;

impl Game for Go {
    type Position = Board;

    fn id(&self) -> &'static str {
        "go"
    }

    fn name(&self) -> &'static str {
        "Go"
    }

    fn initial_position(&self) -> Board {
        Board::new(19)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_counts_are_exact() {
        assert_eq!(Board::new(19).point_count(), 361);
        assert_eq!(Board::new(9).point_count(), 81);
    }

    #[test]
    fn empty_board_has_all_legal_moves() {
        let board = Board::new(9);
        assert_eq!(board.legal_moves(Color::Black).len(), 81);
    }

    #[test]
    fn capture_removes_surrounded_stone() {
        let mut board = Board::new(9);
        board.play((4, 4), Color::White).unwrap();
        board.play((3, 4), Color::Black).unwrap();
        board.play((5, 4), Color::Black).unwrap();
        board.play((4, 3), Color::Black).unwrap();
        let captured = board.play((4, 5), Color::Black).unwrap();
        assert_eq!(captured, vec![(4, 4)]);
    }

    #[test]
    fn suicide_illegal_unless_it_captures() {
        // Pure suicide: no capture available.
        let mut board = Board::new(9);
        board.play((0, 1), Color::White).unwrap();
        board.play((1, 0), Color::White).unwrap();
        assert_eq!(board.play((0, 0), Color::Black), Err(MoveError::Suicide));

        // The same point becomes legal when the move captures.
        let mut board2 = Board::new(9);
        board2.play((0, 1), Color::White).unwrap();
        board2.play((1, 0), Color::White).unwrap();
        board2.play((2, 0), Color::Black).unwrap();
        board2.play((0, 2), Color::Black).unwrap();
        board2.play((1, 1), Color::Black).unwrap();
        let captured = board2.play((0, 0), Color::Black).unwrap();
        assert_eq!(captured.len(), 2);
    }

    #[test]
    fn ko_forbids_immediate_recapture() {
        let mut board = Board::new(9);
        // Black surrounds point (4,4); White surrounds point (4,5).
        board.play((3, 5), Color::Black).unwrap();
        board.play((5, 5), Color::Black).unwrap();
        board.play((4, 6), Color::Black).unwrap();
        board.play((3, 4), Color::White).unwrap();
        board.play((5, 4), Color::White).unwrap();
        board.play((4, 3), Color::White).unwrap();
        board.play((4, 5), Color::White).unwrap();

        // Black captures the white stone at (4,5).
        let captured = board.play((4, 4), Color::Black).unwrap();
        assert_eq!(captured, vec![(4, 5)]);

        // White's immediate recapture would recreate the prior position.
        assert_eq!(board.play((4, 5), Color::White), Err(MoveError::Ko));
    }
}
