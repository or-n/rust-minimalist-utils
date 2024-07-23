use crate::eval::*;
use crate::update::*;
use num::range::*;

pub trait Invert {
    fn invert(&self) -> Self;
}

macro_rules! min_max {
    (   $my:ident, $enemy:ident,
        $better:ident, $better_than: tt,
        $my_f:ident, $enemy_f:ident
    ) => {
        pub fn $my_f<State, Player, Move, UpdateError, Score>(
            depth: usize,
            player: Player,
            mut pruning: Range<Score>,
            state: State,
        ) -> Result<(Option<Move>, Score), UpdateError>
        where
            State: Copy + AvailableMoves<Move>
                + Update<(Player, Move), UpdateError> + Eval<Score>,
            Move: Copy,
            Score: Ord + Copy,
            Player: Copy + Invert,
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
                        state.updated((player, best_move))?.score();
                    if prune(best_score, &mut pruning) {
                        return Ok((Some(best_move), best_score));
                    }
                    for this_move in moves {
                        let (_, score) = $enemy_f(
                            depth - 1,
                            player.invert(),
                            pruning,
                            state.updated((player, this_move))?,
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
