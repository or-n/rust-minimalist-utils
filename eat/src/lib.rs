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
