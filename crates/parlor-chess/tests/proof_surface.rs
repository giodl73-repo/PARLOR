use parlor_chess::{Board, FenError};
use std::collections::BTreeMap;

fn proof() -> BTreeMap<&'static str, &'static str> {
    include_str!("fixtures/fen_proof.txt")
        .lines()
        .map(|line| line.split_once('=').expect("proof fixture uses key=value"))
        .collect()
}

#[test]
fn retained_fen_proof_records_acceptance_and_structured_failure() {
    let proof = proof();
    let board = Board::from_fen(proof["accepted.fen"]).expect("accepted FEN");
    assert_eq!(
        board.legal_moves().len().to_string(),
        proof["accepted.legal_moves"]
    );

    let error = Board::from_fen(proof["rejected.fen"]).unwrap_err();
    assert_eq!(error, FenError::Fields);
    assert_eq!(format!("{error:?}"), proof["rejected.error"]);
}
