use super::{Drop, Eat};

impl<Token: Clone> Eat<&[Token], (), ()> for Token {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Token), ()> {
        if i.is_empty() {
            return Err(());
        }
        Ok((&i[1..], i[0].clone()))
    }
}

impl<Token: Eq> Drop<&[Token], ()> for Token {
    fn drop(self, i: &[Token]) -> Result<&[Token], ()> {
        if i.is_empty() || i[0] != self {
            return Err(());
        }
        Ok(&i[1..])
    }
}
