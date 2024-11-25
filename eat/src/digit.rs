use super::{Eat, EatMany};

pub struct Digit(u32);

impl Eat<&str, (), ()> for Digit {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (i, c) = char::eat(i, ())?;
        let digit = c.to_digit(10).ok_or(())?;
        Ok((i, Digit(digit)))
    }
}

impl Eat<&str, (), ()> for u32 {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (i, digits) = Digit::eat_many(i, ());
        if digits.is_empty() {
            return Err(());
        }
        let n = digits.into_iter().fold(0, |r, digit| r * 10 + digit.0);
        Ok((i, n))
    }
}
