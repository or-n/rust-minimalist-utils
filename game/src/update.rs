pub trait AvailableMoves<Move> {
    fn available_moves(&self) -> Vec<Move>;
}

pub trait Update<Player, Move, MoveError, R>
where
    Self: Sized,
{
    fn update(
        &mut self,
        player: Player,
        m: Move,
    ) -> Result<R, MoveError>;
}

pub fn updated<State, Player, Move, MoveError, R>(
    state: State,
    player: Player,
    m: Move,
) -> Result<State, MoveError>
where
    State: Clone + Update<Player, Move, MoveError, R>,
{
    let mut new = state.clone();
    let _ = new.update(player, m)?;
    Ok(new)
}
