use crate::eval::*;
use crate::update::*;
use num::range::*;

macro_rules! min_max {
    (   $my:ident, $enemy:ident,
        $better:ident, $better_than: tt,
        $my_f:ident, $enemy_f:ident
    ) => {
        pub fn $my_f<State, Move, MoveError, Score, R>(
            depth: usize,
            mut pruning: Range<Score>,
            state: State,
        ) -> Result<(Option<Move>, Score), MoveError>
        where
            State: Copy + AvailableMoves<Move>
                + Update<(), Move, MoveError, R> + Eval<Score>,
            Move: Copy,
            Score: Ord + Copy,
        {
            if depth == 0 {
                return Ok((None, state.score()));
            }
            let mut moves = state.available_moves().into_iter();
            let prune = |score, pruning: &mut Range<Score>| {
                if score $better_than pruning.$enemy {
                    return true;
                }
                pruning.$my = pruning.$my.$better(score);
                false
            };
            match moves.next() {
                Some(mut best_move) => {
                    let mut best_score =
                        updated(state, (), best_move)?.score();
                    if prune(best_score, &mut pruning) {
                        return Ok((Some(best_move), best_score));
                    }
                    for this_move in moves {
                        let (_, score) = $enemy_f(
                            depth - 1,
                            pruning,
                            updated(state, (), this_move)?,
                        )?;
                        if score $better_than best_score {
                            best_score = score;
                            best_move = this_move;
                            if prune(score, &mut pruning) {
                                break;
                            }
                        }
                    }
                    Ok((Some(best_move), best_score))
                }
                _ => Ok((None, state.score())),
            }
        }
    };
}

min_max! {start, end, max, >, maximize, minimize}
min_max! {end, start, min, <, minimize, maximize}
