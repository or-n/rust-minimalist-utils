use crate::traits::*;
use array::uninitialized;
use std::{iter::Sum, ops::Sub};

pub struct SmoothGridNoise<const GRID_SIZE: usize, P> {
    grid: [P; GRID_SIZE],
}

impl<const GRID_SIZE: usize, P> Gen for SmoothGridNoise<GRID_SIZE, P>
where
    P: Gen,
{
    fn generate() -> Self {
        uninitialized!(grid, GRID_SIZE);
        for i in 0..GRID_SIZE {
            grid[i] = P::generate();
        }
        SmoothGridNoise { grid }
    }
}

impl<const GRID_SIZE: usize, RatioP> SmoothGridNoise<GRID_SIZE, RatioP> {
    pub fn noise<P>(&self, index: RatioP) -> RatioP
    where
        RatioP: Copy + Coords<P> + Sub<P, Output = RatioP> + Sum + Dot,
        P: Copy + ToIndex,
    {
        index
            .coords()
            .iter()
            .map(|coord| self.grid[coord.index()].dot(index - *coord))
            .sum()
    }
}
