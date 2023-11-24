use crate::game::EngineGame;
use crate::options::EngineOptions;

use super::{Control, Reporter, SearchRestrictions, Strategy, TimeControl};

#[derive(Default)]
pub struct TopEvalStrategy;

impl<TCx: Control, TRx: Reporter> Strategy<TCx, TRx> for TopEvalStrategy {
    fn go(
        &mut self,
        game: &mut EngineGame,
        _time_control: &TimeControl,
        _restrictions: &SearchRestrictions,
        _options: &EngineOptions,
        control: TCx,
        reporter: TRx,
    ) {
        let mut moves = game.moves();

        moves.sort_unstable_by_key(|m| {
            game.make_move(m);
            let result = game.eval;
            game.undo_move();
            result
        });

        let mv = *moves.first().expect("Could not find a legal move");

        reporter.best_move(mv);
        control.stop();
    }
}
