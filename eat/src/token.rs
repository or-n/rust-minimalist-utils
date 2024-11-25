pub trait Eat<Token, Error, Data>
where
    Self: Sized,
{
    fn eat(i: &[Token], data: Data) -> Result<(&[Token], Self), Error>;
}

pub trait EatMany<Token, Error, Data>
where
    Self: Sized,
{
    fn eat_many(i: &[Token], data: Data) -> (&[Token], Vec<Self>);
}

impl<Token, Error, Data, T> EatMany<Token, Error, Data> for T
where
    T: Eat<Token, Error, Data>,
    Data: Copy,
{
    fn eat_many(mut i: &[Token], data: Data) -> (&[Token], Vec<T>) {
        let mut results = vec![];
        while let Ok((new_i, item)) = T::eat(i, data) {
            i = new_i;
            results.push(item)
        }
        (i, results)
    }
}

pub trait Drop<Token, Error>
where
    Self: Sized,
{
    fn drop(self, tokens: &[Token]) -> Result<&[Token], Error>;
}

pub trait DropMany<Token, Error>
where
    Self: Sized,
{
    fn drop_many(self, tokens: &[Token]) -> (&[Token], usize);
}

impl<Token, Error, T> DropMany<Token, Error> for T
where
    T: Drop<Token, Error> + Copy,
{
    fn drop_many(self, mut i: &[Token]) -> (&[Token], usize) {
        let mut count = 0;
        while let Ok(new_i) = self.drop(i) {
            i = new_i;
            count += 1;
        }
        (i, count)
    }
}

impl<Token: Clone> Eat<Token, (), ()> for Token {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Token), ()> {
        if i.is_empty() {
            return Err(());
        }
        Ok((&i[1..], i[0].clone()))
    }
}

impl<Token: Eq> Drop<Token, ()> for Token {
    fn drop(self, i: &[Token]) -> Result<&[Token], ()> {
        if i.is_empty() || i[0] != self {
            return Err(());
        }
        Ok(&i[1..])
    }
}
