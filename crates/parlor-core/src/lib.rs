#![forbid(unsafe_code)]

/// A qualitative label describing the strength of evidence behind a result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceLabel {
    Proven,
    Cited,
    Heuristic,
    Estimated,
}

impl EvidenceLabel {
    /// Returns `true` only for [`EvidenceLabel::Proven`].
    pub fn is_proven(self) -> bool {
        matches!(self, EvidenceLabel::Proven)
    }
}

/// The product-neutral contract shared by every game in the parlor.
pub trait Game {
    type Position;

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn initial_position(&self) -> Self::Position;
}

/// A game that supports perft (performance test) node counting.
pub trait Perft: Game {
    fn perft(&self, pos: &Self::Position, depth: u32) -> u64;
}

/// A known perft benchmark with an expected node count from a cited source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerftBenchmark {
    pub name: &'static str,
    pub position: &'static str,
    pub depth: u32,
    pub expected_nodes: u64,
    pub source: &'static str,
}

/// The result of running a perft benchmark and observing a node count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerftCheck {
    pub benchmark: PerftBenchmark,
    pub observed_nodes: u64,
    pub label: EvidenceLabel,
}

impl PerftCheck {
    /// Returns `true` when the observed node count matches the expected count.
    pub fn passed(&self) -> bool {
        self.observed_nodes == self.benchmark.expected_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_label_is_proven() {
        assert!(EvidenceLabel::Proven.is_proven());
        assert!(!EvidenceLabel::Heuristic.is_proven());
    }

    #[test]
    fn perft_check_passed() {
        let benchmark = PerftBenchmark {
            name: "startpos",
            position: "initial",
            depth: 1,
            expected_nodes: 20,
            source: "reference",
        };

        let matching = PerftCheck {
            benchmark: benchmark.clone(),
            observed_nodes: 20,
            label: EvidenceLabel::Proven,
        };
        assert!(matching.passed());

        let mismatched = PerftCheck {
            benchmark,
            observed_nodes: 19,
            label: EvidenceLabel::Heuristic,
        };
        assert!(!mismatched.passed());
    }
}
