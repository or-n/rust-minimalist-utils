#[derive(Debug)]
pub enum CharError {
    Missing,
    Mismatch { expected: char, got: char },
}

pub trait Eat<Error, Data>
where
    Self: Sized,
{
    fn eat(i: &str, data: Data) -> Result<(&str, Self), Error>;
}

pub trait EatMany<Error, Data>
where
    Self: Sized,
{
    fn eat_many(i: &str, data: Data) -> (&str, Vec<Self>);
}

impl<Error, Data, T> EatMany<Error, Data> for T
where
    T: Eat<Error, Data>,
    Data: Copy,
{
    fn eat_many(mut i: &str, data: Data) -> (&str, Vec<T>) {
        let mut results = vec![];
        while let Ok((new_i, item)) = T::eat(i, data) {
            i = new_i;
            results.push(item)
        }
        (i, results)
    }
}

pub trait Drop<Error>
where
    Self: Sized,
{
    fn drop(self, i: &str) -> Result<&str, Error>;
}

pub trait DropMany<Error>
where
    Self: Sized,
{
    fn drop_many(self, i: &str) -> (&str, usize);
}

impl<Error, T> DropMany<Error> for T
where
    T: Drop<Error> + Copy,
{
    fn drop_many(self, mut i: &str) -> (&str, usize) {
        let mut count = 0;
        while let Ok(new_i) = self.drop(i) {
            i = new_i;
            count += 1;
        }
        (i, count)
    }
}

impl Eat<(), ()> for char {
    fn eat(i: &str, _data: ()) -> Result<(&str, char), ()> {
        let c = i.chars().next().ok_or(())?;
        Ok((&i[c.len_utf8()..], c))
    }
}

impl Drop<()> for char {
    fn drop(self, i: &str) -> Result<&str, ()> {
        let c = i.chars().next().ok_or(())?;
        if c != self {
            return Err(());
        }
        Ok(&i[c.len_utf8()..])
    }
}

impl<'b> Drop<()> for &'b str {
    fn drop(self, mut i: &str) -> Result<&str, ()> {
        for c in self.chars() {
            i = c.drop(i)?;
        }
        Ok(i)
    }
}
