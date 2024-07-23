pub trait AvailableMoves<Move> {
    fn available_moves(&self) -> Vec<Move>;
}

pub trait Update<Move, Error>
where
    Self: Sized,
{
    fn update(&mut self, m: Move) -> Option<Error>;
}

pub trait Updated<Move, Error>
where
    Self: Sized,
{
    fn updated(&self, m: Move) -> Result<Self, Error>;
}

impl<State, Move, Error> Updated<Move, Error> for State
where
    State: Clone + Update<Move, Error>,
{
    fn updated(&self, m: Move) -> Result<Self, Error> {
        let mut new = self.clone();
        new.update(m).map_or(Ok(()), Err)?;
        Ok(new)
    }
}
