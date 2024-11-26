pub mod digit;
pub mod text;
pub mod token;

pub trait Eat<Input, Error, Data>
where
    Self: Sized,
{
    fn eat(i: Input, data: Data) -> Result<(Input, Self), Error>;
}

pub trait EatMany<Input, Error, Data>
where
    Self: Sized,
{
    fn eat_many(i: Input, data: Data) -> (Input, Vec<Self>);
}

impl<Input, Error, Data, T> EatMany<Input, Error, Data> for T
where
    T: Eat<Input, Error, Data>,
    Data: Copy,
    Input: Copy,
{
    fn eat_many(mut i: Input, data: Data) -> (Input, Vec<T>) {
        let mut results = vec![];
        while let Ok((new_i, item)) = T::eat(i, data) {
            i = new_i;
            results.push(item)
        }
        (i, results)
    }
}

pub trait Drop<Input, Error>
where
    Self: Sized,
{
    fn drop(self, i: Input) -> Result<Input, Error>;
}

pub trait DropMany<Input, Error>
where
    Self: Sized,
{
    fn drop_many(self, i: Input) -> (Input, usize);
}

impl<Input, Error, T> DropMany<Input, Error> for T
where
    T: Drop<Input, Error> + Copy,
    Input: Copy,
{
    fn drop_many(self, mut i: Input) -> (Input, usize) {
        let mut count = 0;
        while let Ok(new_i) = self.drop(i) {
            i = new_i;
            count += 1;
        }
        (i, count)
    }
}

impl Eat<&[u8], (), ()> for u32 {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, b0) = u8::eat(i, ())?;
        let (i, b1) = u8::eat(i, ())?;
        let (i, b2) = u8::eat(i, ())?;
        let (i, b3) = u8::eat(i, ())?;
        let x = u32::from_be_bytes([b0, b1, b2, b3]);
        Ok((i, x))
    }
}

pub struct SeqN<T>(pub Vec<T>);

impl<'a, T> Eat<&'a [u8], (), ()> for SeqN<T>
where
    T: Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (mut i, n) = u32::eat(i, ())?;
        let mut r = Vec::new();
        for _ in 0..n {
            let (new_i, x) = T::eat(i, ())?;
            i = new_i;
            r.push(x);
        }
        Ok((i, SeqN(r)))
    }
}
