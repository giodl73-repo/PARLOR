#![forbid(unsafe_code)]

//! `parlor-backgammon`: a standard backgammon rules and analysis kernel.
//!
//! Board model: 24 points (absolute indices `0..=23`) plus each player's bar
//! and borne-off count, 15 checkers per side. White advances toward index 0 and
//! bears off below it; Black advances toward index 23 and bears off above it.
//! Each player advances toward their own home board. The pip count is the sum,
//! over a player's 15 checkers, of each checker's distance to bear-off. The
//! canonical opening pip count is 167 per player.

use parlor_core::Game;

/// The two players.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    /// The opponent of this player.
    pub fn opponent(self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

/// A backgammon position.
///
/// `points[i]` encodes point `i` (absolute index `0..=23`): a positive value is
/// that many White checkers, a negative value that many Black checkers, zero is
/// empty. White checkers count positively, Black checkers negatively.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    points: [i8; 24],
    bar_white: u8,
    bar_black: u8,
    off_white: u8,
    off_black: u8,
}

/// A legal play: the resulting board after applying a roll, plus how many
/// opposing blots were hit during the sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Play {
    pub board: Board,
    pub hits: u8,
}

struct Cand {
    board: Board,
    used: Vec<u8>,
    hits: u32,
}

impl Board {
    /// The standard opening setup (167 pips per player).
    pub fn start() -> Board {
        let mut points = [0i8; 24];
        // White checkers (positive): points 6, 8, 13, 24 (indices 5, 7, 12, 23).
        points[5] = 5;
        points[7] = 3;
        points[12] = 5;
        points[23] = 2;
        // Black checkers (negative): mirror image.
        points[18] = -5;
        points[16] = -3;
        points[11] = -5;
        points[0] = -2;
        Board {
            points,
            bar_white: 0,
            bar_black: 0,
            off_white: 0,
            off_black: 0,
        }
    }

    /// Pip count for `player`: the sum over the player's 15 checkers of each
    /// checker's distance to bear-off (a checker on the bar counts as 25 pips).
    pub fn pip_count(&self, player: Player) -> u32 {
        let mut sum: u32 = 0;
        match player {
            Player::White => {
                for i in 0..24 {
                    if self.points[i] > 0 {
                        sum += self.points[i] as u32 * (i as u32 + 1);
                    }
                }
                sum += self.bar_white as u32 * 25;
            }
            Player::Black => {
                for i in 0..24 {
                    if self.points[i] < 0 {
                        sum += (-self.points[i]) as u32 * (24 - i as u32);
                    }
                }
                sum += self.bar_black as u32 * 25;
            }
        }
        sum
    }

    fn all_home(&self, player: Player) -> bool {
        match player {
            Player::White => {
                if self.bar_white > 0 {
                    return false;
                }
                for i in 6..24 {
                    if self.points[i] > 0 {
                        return false;
                    }
                }
                true
            }
            Player::Black => {
                if self.bar_black > 0 {
                    return false;
                }
                for i in 0..18 {
                    if self.points[i] < 0 {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn is_highest_white(&self, i: usize) -> bool {
        for j in (i + 1)..24 {
            if self.points[j] > 0 {
                return false;
            }
        }
        true
    }

    fn is_lowest_black(&self, i: usize) -> bool {
        for j in 0..i {
            if self.points[j] < 0 {
                return false;
            }
        }
        true
    }

    /// All legal single-checker moves for `player` using one die of value `die`,
    /// returning each resulting board and whether it hit a blot.
    fn single_moves(&self, player: Player, die: u8) -> Vec<(Board, bool)> {
        let mut res: Vec<(Board, bool)> = Vec::new();
        let d = die as i32;
        match player {
            Player::White => {
                if self.bar_white > 0 {
                    let to = (24 - d) as usize;
                    if self.points[to] > -2 {
                        let hit = self.points[to] == -1;
                        let mut b = self.clone();
                        b.bar_white -= 1;
                        if hit {
                            b.points[to] = 1;
                            b.bar_black += 1;
                        } else {
                            b.points[to] += 1;
                        }
                        res.push((b, hit));
                    }
                    return res;
                }
                let home = self.all_home(Player::White);
                for i in 0..24usize {
                    if self.points[i] > 0 {
                        let t = i as i32 - d;
                        if t >= 0 {
                            let to = t as usize;
                            if self.points[to] > -2 {
                                let hit = self.points[to] == -1;
                                let mut b = self.clone();
                                b.points[i] -= 1;
                                if hit {
                                    b.points[to] = 1;
                                    b.bar_black += 1;
                                } else {
                                    b.points[to] += 1;
                                }
                                res.push((b, hit));
                            }
                        } else if home && (t == -1 || self.is_highest_white(i)) {
                            let mut b = self.clone();
                            b.points[i] -= 1;
                            b.off_white += 1;
                            res.push((b, false));
                        }
                    }
                }
            }
            Player::Black => {
                if self.bar_black > 0 {
                    let to = (d - 1) as usize;
                    if self.points[to] < 2 {
                        let hit = self.points[to] == 1;
                        let mut b = self.clone();
                        b.bar_black -= 1;
                        if hit {
                            b.points[to] = -1;
                            b.bar_white += 1;
                        } else {
                            b.points[to] -= 1;
                        }
                        res.push((b, hit));
                    }
                    return res;
                }
                let home = self.all_home(Player::Black);
                for i in 0..24usize {
                    if self.points[i] < 0 {
                        let t = i as i32 + d;
                        if t <= 23 {
                            let to = t as usize;
                            if self.points[to] < 2 {
                                let hit = self.points[to] == 1;
                                let mut b = self.clone();
                                b.points[i] += 1;
                                if hit {
                                    b.points[to] = -1;
                                    b.bar_white += 1;
                                } else {
                                    b.points[to] -= 1;
                                }
                                res.push((b, hit));
                            }
                        } else if home && (t == 24 || self.is_lowest_black(i)) {
                            let mut b = self.clone();
                            b.points[i] += 1;
                            b.off_black += 1;
                            res.push((b, false));
                        }
                    }
                }
            }
        }
        res
    }

    fn gen(
        &self,
        player: Player,
        remaining: Vec<u8>,
        used: Vec<u8>,
        hits: u32,
        out: &mut Vec<Cand>,
    ) {
        let mut moved = false;
        let mut seen: Vec<u8> = Vec::new();
        for k in 0..remaining.len() {
            let d = remaining[k];
            if seen.contains(&d) {
                continue;
            }
            seen.push(d);
            for (nb, hit) in self.single_moves(player, d) {
                moved = true;
                let mut rem = remaining.clone();
                rem.remove(k);
                let mut u = used.clone();
                u.push(d);
                nb.gen(player, rem, u, hits + hit as u32, out);
            }
        }
        if !moved {
            out.push(Cand {
                board: self.clone(),
                used: used.clone(),
                hits,
            });
        }
    }

    /// Every distinct legal play for the roll `(die_a, die_b)` under standard
    /// rules: bar checkers re-enter first; doubles are four moves of that value;
    /// you must play both dice whenever some sequence does, and if only one die
    /// can be played you must play the higher one when possible; bearing off is
    /// allowed only once all of the player's checkers are home.
    pub fn legal_plays(&self, player: Player, die_a: u8, die_b: u8) -> Vec<Play> {
        let mut cands: Vec<Cand> = Vec::new();
        let dice = if die_a == die_b {
            vec![die_a, die_a, die_a, die_a]
        } else {
            vec![die_a, die_b]
        };
        self.gen(player, dice, Vec::new(), 0, &mut cands);

        let max_used = cands.iter().map(|c| c.used.len()).max().unwrap_or(0);
        if max_used == 0 {
            return Vec::new();
        }

        let mut filtered: Vec<&Cand> = cands.iter().filter(|c| c.used.len() == max_used).collect();

        if max_used == 1 && die_a != die_b {
            let higher = die_a.max(die_b);
            if filtered.iter().any(|c| c.used[0] == higher) {
                filtered.retain(|c| c.used[0] == higher);
            }
        }

        let mut plays: Vec<Play> = Vec::new();
        for c in filtered {
            if !plays.iter().any(|p| p.board == c.board) {
                plays.push(Play {
                    board: c.board.clone(),
                    hits: c.hits as u8,
                });
            }
        }
        plays
    }
}

/// One of the 21 distinct unordered dice rolls with its probability over the 36
/// equally-likely ordered outcomes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Roll {
    pub die_a: u8,
    pub die_b: u8,
    pub probability: f64,
}

/// Dice helper exposing the distinct rolls and roll statistics.
#[derive(Clone, Copy, Debug)]
pub struct Dice;

impl Dice {
    /// The 21 distinct unordered rolls. Each double has probability `1/36`; each
    /// non-double has probability `2/36`; the probabilities sum to `1`.
    pub fn distinct_rolls() -> Vec<Roll> {
        let mut v = Vec::with_capacity(21);
        for a in 1..=6u8 {
            for b in a..=6u8 {
                let probability = if a == b { 1.0 / 36.0 } else { 2.0 / 36.0 };
                v.push(Roll {
                    die_a: a,
                    die_b: b,
                    probability,
                });
            }
        }
        v
    }

    /// The mean roll value, `49.0 / 6.0`. A double is worth four times its face.
    pub fn expected_pips() -> f64 {
        49.0 / 6.0
    }
}

/// The backgammon game, implementing [`parlor_core::Game`].
pub struct Backgammon;

impl Game for Backgammon {
    type Position = Board;

    fn id(&self) -> &'static str {
        "backgammon"
    }

    fn name(&self) -> &'static str {
        "Backgammon"
    }

    fn initial_position(&self) -> Self::Position {
        Board::start()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_pip_counts_are_167() {
        assert_eq!(Board::start().pip_count(Player::White), 167);
        assert_eq!(Board::start().pip_count(Player::Black), 167);
    }

    #[test]
    fn distinct_rolls_distribution_is_exact() {
        let rolls = Dice::distinct_rolls();
        assert_eq!(rolls.len(), 21);

        // The 21 unordered rolls cover exactly the 36 equally-likely ordered
        // outcomes: six doubles (1 each) plus fifteen non-doubles (2 each).
        let ordered: u32 = rolls
            .iter()
            .map(|r| (r.probability * 36.0).round() as u32)
            .sum();
        assert_eq!(ordered, 36);

        let total: f64 = rolls.iter().map(|r| r.probability).sum();
        assert!((total - 1.0).abs() < 1e-9);
    }

    #[test]
    fn expected_pips_is_49_over_6() {
        assert_eq!(Dice::expected_pips(), 49.0 / 6.0);
    }

    #[test]
    fn opening_has_a_legal_play_for_3_1() {
        let plays = Board::start().legal_plays(Player::White, 3, 1);
        assert!(!plays.is_empty());
    }

    #[test]
    fn landing_on_a_blot_sends_it_to_the_bar() {
        let mut b = Board {
            points: [0i8; 24],
            bar_white: 0,
            bar_black: 0,
            off_white: 0,
            off_black: 0,
        };
        // White checkers outside home (so no bearing off), and a lone Black blot
        // at index 5 reachable from index 8 with a 3.
        b.points[8] = 1;
        b.points[20] = 1;
        b.points[5] = -1;

        let plays = b.legal_plays(Player::White, 3, 1);
        let hit_play = plays.iter().find(|p| p.hits >= 1);
        assert!(hit_play.is_some());
        assert!(hit_play.unwrap().board.bar_black >= 1);
    }
}
