use chess::game::GameStatus;
use chess::{game::Game, moves::Move};
use rand::Rng;

use crate::eval::Eval;
use crate::search::time_control::TimeControl;
use crate::{
    eval::{self},
    strategy::Reporter,
};

use super::{move_ordering, negamax_eval::NegamaxEval, SearchState};

pub fn negamax(
    game: &Game,
    depth: u8,
    state: &mut SearchState,
    time_control: &TimeControl,
    _reporter: &impl Reporter,
) -> Result<(Move, Vec<Move>, NegamaxEval), ()> {
    let mut best_move: Option<Move> = None;
    let mut best_line: Option<Vec<Move>> = None;
    let mut best_score = NegamaxEval::MIN;

    let mut root_moves = game.legal_moves();
    let best_previous_root_move = state.best_pv.as_ref().and_then(|pv| pv.first().copied());

    move_ordering::order_moves(game, &mut root_moves, best_previous_root_move);

    for mv in &root_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let mut line: Vec<Move> = vec![];

        let move_score = -negamax_inner(
            &game_after_move,
            NegamaxEval::MIN,
            NegamaxEval::MAX,
            depth - 1,
            1,
            &mut line,
            time_control,
            state,
        )?;

        if move_score > best_score {
            line.insert(0, *mv);
            best_score = move_score;
            best_line = Some(line);
            best_move = Some(*mv);
        }
    }

    Ok((best_move.unwrap(), best_line.unwrap(), best_score))
}

fn negamax_inner(
    game: &Game,
    mut alpha: NegamaxEval,
    beta: NegamaxEval,
    depth: u8,
    plies: u8,
    pv: &mut Vec<Move>,
    time_control: &TimeControl,
    state: &mut SearchState,
) -> Result<NegamaxEval, ()> {
    state.max_depth_reached = state.max_depth_reached.max(plies);

    if depth == 0 {
        pv.clear();

        state.nodes_visited += 1;

        // Introduce a tiny bit of noise into the evaluation function to add some variation
        // to play in the same situations where we'd otherwise always pick the first move
        // with the same score.
        let eval_noise = rand::thread_rng().gen_range(0..10);
        let eval = eval::eval(game) + Eval(eval_noise);

        return Ok(NegamaxEval::from_eval(eval, game.player));
    }

    let mut line: Vec<Move> = vec![];

    let game_status = game.game_status();
    if let Some(status) = game_status {
        pv.clear();

        return Ok(match status {
            GameStatus::Won => NegamaxEval::mate_in(plies),
            GameStatus::Lost => NegamaxEval::mated_in(plies),
            GameStatus::Stalemate => NegamaxEval::DRAW,
        });
    }

    // Check periodically to see if we're out of time. If we are, we shouldn't continue the search
    // so we return Err to signal to the caller that the search did not complete.
    if state.nodes_visited % 10000 == 0 && time_control.should_stop() {
        return Err(());
    }

    let mut legal_moves = game.legal_moves();
    move_ordering::order_moves(game, &mut legal_moves, None);

    for mv in &legal_moves {
        let game_after_move = game.make_move(mv).unwrap();
        state.nodes_visited += 1;

        let move_score = -negamax_inner(
            &game_after_move,
            -beta,
            -alpha,
            depth - 1,
            plies + 1,
            &mut line,
            time_control,
            state,
        )?;

        if move_score >= beta {
            state.beta_cutoffs += 1;
            return Ok(beta);
        }

        if move_score > alpha {
            alpha = move_score;

            pv.clear();
            pv.push(*mv);
            pv.extend_from_slice(&line);
        }
    }

    Ok(alpha)
}
