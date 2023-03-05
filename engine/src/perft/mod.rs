use chess::game::Game;

#[must_use]
pub fn perft(depth: u8, game: &Game) -> usize {
    if depth == 1 {
        return game.legal_moves().len();
    }

    game.legal_moves()
        .iter()
        .map(|m| perft(depth - 1, &game.make_move(m).unwrap()))
        .sum()
}

pub fn perft_div(depth: u8, game: &Game) {
    let root_moves = game.legal_moves();
    let mut all = 0;

    for mv in root_moves {
        let number_for_mv = perft(depth - 1, &game.make_move(&mv).unwrap());

        println!("{mv:?} {number_for_mv}");
        all += number_for_mv;
    }

    println!();
    println!("{all}");
}